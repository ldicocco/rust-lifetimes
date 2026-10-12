#!/usr/bin/env python3
"""Verify every program of the book.

Two kinds of artifacts are checked:

Rust examples (code/examples/*.rs and code/crates/*/examples/*.rs)
    carry a directive on one of their first lines:
        // harness: run [timeout=SECONDS] [args="..."]   must exit with code 0
        // harness: build-only                            only compiled
    They must compile without a single warning. The standard output of every
    `run` example is compared with the snapshot in code/output/<name>.txt, which
    the book includes. `--bless` rewrites the snapshots from the current output.

Rust programs that must NOT compile (code/compile_fail/*.rs)
    carry the error codes they must produce:
        // harness: compile-fail E0597 [E0505 ...] [edition=2021]
    or, for diagnostics that have no error code (many lifetime errors: "lifetime
    may not live long enough", "implementation of `Fn` is not general enough",
    or a lint made an error with #![deny(...)]), a quoted text the output must
    contain:
        // harness: compile-fail "lifetime may not live long enough"
    Codes and texts can be mixed. They are compiled with rustc on their own, as a
    library, with the edition of the workspace (2024) unless `edition=` says
    otherwise; the diagnostics are compared with code/compile_fail/<name>.stderr
    (included by the book). `--bless` rewrites those files. `--codes-only`
    checks only the codes and texts. Paths into the local copy of the standard
    library's source are written as rustc's machine-independent
    /rustc/<commit-hash>/library/... .

Usage:
    tools/run_examples.py                  # every example and compile-fail program
    tools/run_examples.py --only ch07      # only artifacts of chapter 7
    tools/run_examples.py --rust           # only the runnable examples
    tools/run_examples.py --compile-fail   # only the programs that must be rejected
    tools/run_examples.py --tests          # also run `cargo test` for every package
    tools/run_examples.py --only ch07 --bless

Honours CARGO_TARGET_DIR.
"""

from __future__ import annotations

import argparse
import functools
import os
import re
import shlex
import subprocess
import sys
import tempfile
import time
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
RUST = ROOT / "code"
OUTPUT = RUST / "output"
COMPILE_FAIL = RUST / "compile_fail"
HARNESS_RE = re.compile(r"^\s*//\s*harness:\s*(\S+)(.*)$")
ERROR_CODE_RE = re.compile(r"E\d{4}")


def default_edition() -> str:
    cargo = tomllib.loads((RUST / "Cargo.toml").read_text())
    return str(cargo["package"]["edition"])


@dataclass
class Program:
    name: str
    package: str
    mode: str          # run | build-only
    opts: dict = field(default_factory=dict)
    path: Path = Path()
    root_package: bool = False


@dataclass
class Result:
    kind: str
    name: str
    ok: bool
    detail: str = ""
    seconds: float = 0.0


def parse_directive(path: Path) -> tuple[str, list[str], dict] | None:
    try:
        head = path.read_text().splitlines()[:5]
    except OSError:
        return None
    for line in head:
        m = HARNESS_RE.match(line)
        if m:
            words, opts = [], {}
            for tok in shlex.split(m.group(2)):
                if "=" in tok:
                    k, v = tok.split("=", 1)
                    opts[k] = v
                else:
                    words.append(tok)
            return m.group(1), words, opts
    return None


def package_name(crate_dir: Path) -> str:
    return tomllib.loads((crate_dir / "Cargo.toml").read_text())["package"]["name"]


def discover_rust() -> list[Program]:
    progs: list[Program] = []
    dirs = [(RUST / "examples", package_name(RUST), True)]
    crates = RUST / "crates"
    if crates.is_dir():
        for crate in sorted(crates.iterdir()):
            if (crate / "Cargo.toml").exists() and (crate / "examples").is_dir():
                dirs.append((crate / "examples", package_name(crate), False))
    for ex_dir, pkg, is_root in dirs:
        if not ex_dir.is_dir():
            continue
        for p in sorted(ex_dir.iterdir()):
            src = p / "main.rs" if p.is_dir() else p
            if src.suffix != ".rs" or not src.exists():
                continue
            d = parse_directive(src)
            name = p.stem if p.is_file() else p.name
            if d is None:
                print(f"warning: {src.relative_to(ROOT)} has no harness directive; skipped")
                continue
            progs.append(Program(name, pkg, d[0], d[2], src, is_root))
    return progs


def cargo_env() -> dict:
    env = dict(os.environ)
    env.setdefault("CARGO_TERM_COLOR", "never")
    return env


def run_rust(prog: Program, bless: bool) -> Result:
    t0 = time.time()
    pkg_args = [] if prog.root_package else ["-p", prog.package]
    if prog.mode == "build-only":
        cmd = ["cargo", "build", "-q", *pkg_args, "--example", prog.name]
        res = subprocess.run(cmd, cwd=RUST, capture_output=True, text=True, env=cargo_env())
        ok = res.returncode == 0
        return Result("rust", prog.name, ok, "" if ok else res.stderr[-3000:], time.time() - t0)
    if prog.mode != "run":
        return Result("rust", prog.name, False, f"unknown harness mode '{prog.mode}'")
    build = subprocess.run(
        ["cargo", "build", "-q", *pkg_args, "--example", prog.name],
        cwd=RUST, capture_output=True, text=True, env=cargo_env(),
    )
    if build.returncode != 0:
        return Result("rust", prog.name, False, build.stderr[-3000:], time.time() - t0)
    if build.stderr.strip():
        # Warnings: the book's examples must have none.
        return Result("rust", prog.name, False, "compiler warnings:\n" + build.stderr[-3000:])
    timeout = float(prog.opts.get("timeout", 60))
    args = shlex.split(prog.opts.get("args", ""))
    cmd = ["cargo", "run", "-q", *pkg_args, "--example", prog.name, "--", *args]
    try:
        res = subprocess.run(cmd, cwd=RUST, capture_output=True, text=True,
                             env=cargo_env(), timeout=timeout)
    except subprocess.TimeoutExpired:
        return Result("rust", prog.name, False, f"timed out after {timeout:.0f}s", time.time() - t0)
    secs = time.time() - t0
    if res.returncode != 0:
        return Result("rust", prog.name, False,
                      f"exit code {res.returncode}\n{res.stdout[-1500:]}{res.stderr[-1500:]}", secs)
    snap = OUTPUT / f"{prog.name}.txt"
    if bless:
        OUTPUT.mkdir(parents=True, exist_ok=True)
        snap.write_text(res.stdout)
        return Result("rust", prog.name, True, "blessed", secs)
    if not snap.exists():
        return Result("rust", prog.name, False, f"no snapshot {snap.relative_to(ROOT)} (run with --bless)", secs)
    if snap.read_text() != res.stdout:
        return Result("rust", prog.name, False, diff(snap.read_text(), res.stdout), secs)
    return Result("rust", prog.name, True, "", secs)


def diff(expected: str, actual: str) -> str:
    import difflib

    lines = difflib.unified_diff(expected.splitlines(), actual.splitlines(),
                                 "snapshot", "actual", lineterm="", n=2)
    return "output differs from snapshot:\n" + "\n".join(list(lines)[:40])


# --------------------------------------------------------------------------------------
# compile-fail
# --------------------------------------------------------------------------------------

TRAILER_RE = re.compile(
    r"^(error: aborting due to .*|For more information about (this|an) error.*|"
    r"Some errors have detailed explanations.*|warning: \d+ warnings? emitted)$"
)


@functools.cache
def std_source_paths() -> tuple[str, str]:
    """The local path of the standard library's source (the rust-src component),
    which depends on the machine, and the path rustc shows without it."""
    def rustc(*args: str) -> str:
        return subprocess.run(["rustc", *args], cwd=RUST, capture_output=True,
                              text=True, check=True).stdout
    sysroot = Path(rustc("--print", "sysroot").strip())
    commit = re.search(r"^commit-hash: (\S+)$", rustc("-vV"), re.M).group(1)
    return str(sysroot / "lib" / "rustlib" / "src" / "rust"), f"/rustc/{commit}"


def normalize_stderr(text: str) -> str:
    local, portable = std_source_paths()
    text = text.replace(local, portable)
    lines = [ln.rstrip() for ln in text.splitlines()]
    lines = [ln for ln in lines if not TRAILER_RE.match(ln)]
    while lines and not lines[-1].strip():
        lines.pop()
    return "\n".join(lines) + "\n"


def run_compile_fail(path: Path, bless: bool, codes_only: bool) -> Result:
    t0 = time.time()
    d = parse_directive(path)
    rel = path.relative_to(RUST)
    if d is None or d[0] != "compile-fail" or not d[1]:
        return Result("compile-fail", path.stem, False,
                      "needs a '// harness: compile-fail E0xxx' or "
                      "'// harness: compile-fail \"message\"' directive")
    codes = [w for w in d[1] if ERROR_CODE_RE.fullmatch(w)]
    texts = [w for w in d[1] if not ERROR_CODE_RE.fullmatch(w)]
    edition = d[2].get("edition", default_edition())
    with tempfile.TemporaryDirectory() as tmp:
        cmd = ["rustc", "--edition", edition, "--crate-type", "lib", "--emit=metadata",
               "--color", "never", "-A", "warnings", "--crate-name", path.stem,
               "-o", str(Path(tmp) / "out.rmeta"), str(rel)]
        res = subprocess.run(cmd, cwd=RUST, capture_output=True, text=True)
    secs = time.time() - t0
    if res.returncode == 0:
        return Result("compile-fail", path.stem, False, "compiled successfully, but must not", secs)
    missing = [c for c in codes if f"error[{c}]" not in res.stderr]
    missing += [repr(t) for t in texts if t not in res.stderr]
    if missing:
        return Result("compile-fail", path.stem, False,
                      f"expected {', '.join(missing)}; got:\n{res.stderr[-2500:]}", secs)
    text = normalize_stderr(res.stderr)
    snap = path.with_suffix(".stderr")
    if bless:
        snap.write_text(text)
        return Result("compile-fail", path.stem, True, "blessed", secs)
    if codes_only:
        return Result("compile-fail", path.stem, True, "", secs)
    if not snap.exists():
        return Result("compile-fail", path.stem, False,
                      f"no snapshot {snap.relative_to(ROOT)} (run with --bless)", secs)
    if snap.read_text() != text:
        return Result("compile-fail", path.stem, False, diff(snap.read_text(), text), secs)
    return Result("compile-fail", path.stem, True, "", secs)


# --------------------------------------------------------------------------------------
# package tests
# --------------------------------------------------------------------------------------

def run_tests() -> list[Result]:
    results = []
    pkgs = [package_name(RUST)]
    crates = RUST / "crates"
    if crates.is_dir():
        pkgs += [package_name(c) for c in sorted(crates.iterdir()) if (c / "Cargo.toml").exists()]
    for pkg in pkgs:
        t0 = time.time()
        res = subprocess.run(["cargo", "test", "-q", "-p", pkg], cwd=RUST,
                             capture_output=True, text=True, env=cargo_env())
        ok = res.returncode == 0
        results.append(Result("test", pkg, ok, "" if ok else (res.stdout + res.stderr)[-3000:],
                              time.time() - t0))
    return results


# --------------------------------------------------------------------------------------
# main
# --------------------------------------------------------------------------------------

def selected(name: str, only: str | None) -> bool:
    """Does an artifact name match --only?

    A bare chapter (`ch2`, `ch02`) selects exactly that chapter, so that `ch1` does
    not also select chapters 10-19; an appendix (`appA`) works the same way;
    anything else is a plain name prefix.
    """
    if not only:
        return True
    m = re.fullmatch(r"ch(\d+)", only, re.I)
    if m:
        return name.lower().startswith(f"ch{int(m.group(1)):02d}_")
    m = re.fullmatch(r"app([A-Za-z])", only, re.I)
    if m:
        return name.lower().startswith(f"app{m.group(1).lower()}_")
    return name.startswith(only)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--only", help="only artifacts whose name starts with this prefix (e.g. ch05)")
    ap.add_argument("--rust", action="store_true", help="run the Rust examples")
    ap.add_argument("--compile-fail", action="store_true", help="check the compile-fail programs")
    ap.add_argument("--tests", action="store_true", help="also run cargo test for every package")
    ap.add_argument("--bless", action="store_true", help="rewrite output and diagnostic snapshots")
    ap.add_argument("--codes-only", action="store_true",
                    help="compile-fail: check error codes and texts but not the diagnostic snapshot")
    a = ap.parse_args()
    everything = not (a.rust or a.compile_fail)
    results: list[Result] = []

    if a.rust or everything:
        for prog in discover_rust():
            if not selected(prog.name, a.only):
                continue
            r = run_rust(prog, a.bless)
            results.append(r)
            report(r)
    if a.compile_fail or everything:
        for path in sorted(COMPILE_FAIL.glob("*.rs")):
            if not selected(path.stem, a.only):
                continue
            r = run_compile_fail(path, a.bless, a.codes_only)
            results.append(r)
            report(r)
    if a.tests:
        for r in run_tests():
            results.append(r)
            report(r)

    failed = [r for r in results if not r.ok]
    print(f"\n{len(results) - len(failed)} passed, {len(failed)} failed")
    return 1 if failed else 0


def report(r: Result) -> None:
    status = "ok  " if r.ok else "FAIL"
    extra = f" ({r.detail})" if r.ok and r.detail else ""
    print(f"{status} {r.kind:<13} {r.name:<36} {r.seconds:5.1f}s{extra}")
    if not r.ok and r.detail:
        print("     " + r.detail.replace("\n", "\n     "))
    sys.stdout.flush()


if __name__ == "__main__":
    sys.exit(main())
