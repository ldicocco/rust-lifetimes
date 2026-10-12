#!/usr/bin/env python3
"""Build tool for the book *Rust Lifetimes*.

Sub-commands
    check [FILES...]   validate includes, anchors and image links (default: all of book/src)
    md                 expand includes -> book/chapters/NN-slug.md (numbered titles)
    diagrams           render book/diagrams/*.dot -> book/images/*.svg
    pdf [--only SLUG]  per-chapter PDFs + the full book PDF (pandoc -> Typst)
    all                diagrams + md + pdf
    readme             refresh the table of contents in README.md

Markdown sources live in book/src and use mdBook-style includes:
    {{#include ../../code/examples/ch05_x.rs:anchor}}
    {{#include ../../code/output/ch05_x.txt}}
    {{#include ../../code/compile_fail/ch05_y.stderr}}
    {{#include ../../code/examples/ch05_x.rs:10:20}}
Anchors are delimited by `// ANCHOR: name` / `// ANCHOR_END: name` lines.
"""

from __future__ import annotations

import argparse
import re
import shutil
import subprocess
import sys
import textwrap
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BOOK = ROOT / "book"
SRC = BOOK / "src"
OUT = BOOK / "chapters"
IMAGES = BOOK / "images"
DIAGRAMS = BOOK / "diagrams"
TOOLS = ROOT / "tools"

INCLUDE_RE = re.compile(r"\{\{#include\s+([^}\s]+)\s*\}\}")
ANCHOR_RE = re.compile(r"^\s*(?://|#|--|/\*)\s*ANCHOR(_END)?:\s*([A-Za-z0-9_\-']+)")
HARNESS_RE = re.compile(r"^\s*//\s*harness:")
IMAGE_RE = re.compile(r"!\[[^\]]*\]\(([^)\s]+)(?:\s+\"[^\"]*\")?\)")
FENCE_RE = re.compile(r"^(\s*)(`{3,}|~{3,})(.*)$")

# TeX commands that pandoc cannot convert to Typst, with the replacement to use.
FORBIDDEN_TEX = {
    "\\textsc": "\\textsf{...}",
    "\\parr": "the Unicode character \u214b",
    "\\begin{prooftree}": "nested \\dfrac{...}{...}",
    "\\inferrule": "\\dfrac{premises}{conclusion}\\;\\textsf{Name}",
    "\\atop": "\\dfrac or a sequent written on one line",
}
# Characters the PDF fonts cannot render (none at the moment: the code font
# falls back to New Computer Modern Math for symbols such as Mathlib's identity).
FORBIDDEN_CHARS: dict[str, str] = {}


# --------------------------------------------------------------------------------------
# Book structure
# --------------------------------------------------------------------------------------

@dataclass
class Entry:
    kind: str          # "preface" | "chapter" | "appendix" | "part"
    file: str | None   # source file name in book/src (None for parts)
    title: str         # part title, or fallback chapter title
    label: str = ""    # "1", "2", ..., "A", "B", ...
    part_intro: str | None = None  # source file for a part introduction page


def read_summary() -> list[Entry]:
    """Parse book/src/SUMMARY.md (mdBook format) into an ordered list of entries."""
    summary = SRC / "SUMMARY.md"
    entries: list[Entry] = []
    chapter_no = 0
    appendix_no = 0
    in_appendix = False
    for raw in summary.read_text().splitlines():
        line = raw.strip()
        if not line or line == "# Summary":
            continue
        m_part = re.match(r"^#\s+(.*)$", line)
        if m_part:
            title = m_part.group(1).strip()
            in_appendix = title.lower().startswith("appendi")
            entries.append(Entry("part", None, title))
            continue
        m_link = re.match(r"^(?:[-*]\s+)?\[(.*?)\]\((.*?)\)\s*$", line)
        if not m_link:
            continue
        title, file = m_link.group(1), m_link.group(2)
        if file.startswith("part-"):
            # Part introduction text attached to the preceding part heading.
            if entries and entries[-1].kind == "part":
                entries[-1].part_intro = file
            continue
        if not line.startswith(("-", "*")):
            entries.append(Entry("preface", file, title))
        elif in_appendix:
            label = chr(ord("A") + appendix_no)
            appendix_no += 1
            entries.append(Entry("appendix", file, title, label))
        else:
            chapter_no += 1
            entries.append(Entry("chapter", file, title, str(chapter_no)))
    return entries


# --------------------------------------------------------------------------------------
# Include expansion
# --------------------------------------------------------------------------------------

class IncludeError(Exception):
    pass


def extract(path: Path, selector: str | None) -> str:
    if not path.exists():
        raise IncludeError(f"included file not found: {path}")
    lines = path.read_text().splitlines()
    if selector is None:
        body = lines
    elif re.fullmatch(r"\d*:\d*", selector):
        a, b = selector.split(":")
        start = int(a) - 1 if a else 0
        end = int(b) if b else len(lines)
        body = lines[start:end]
    elif re.fullmatch(r"\d+", selector):
        body = lines[int(selector) - 1:]
    else:
        body, inside, found = [], False, False
        for ln in lines:
            m = ANCHOR_RE.match(ln)
            if m and m.group(2) == selector:
                if m.group(1):  # END
                    inside = False
                else:
                    inside, found = True, True
                continue
            if inside:
                body.append(ln)
        if not found:
            raise IncludeError(f"anchor '{selector}' not found in {path}")
        if inside:
            raise IncludeError(f"anchor '{selector}' has no ANCHOR_END in {path}")
    body = [ln for ln in body if not ANCHOR_RE.match(ln) and not HARNESS_RE.match(ln)]
    # Trim leading/trailing blank lines, then dedent.
    while body and not body[0].strip():
        body.pop(0)
    while body and not body[-1].strip():
        body.pop()
    return textwrap.dedent("\n".join(body))


def expand_includes(md_path: Path, text: str) -> str:
    def repl(m: re.Match) -> str:
        spec = m.group(1)
        parts = spec.split(":", 1)
        file_part = parts[0]
        selector = parts[1] if len(parts) > 1 else None
        target = (md_path.parent / file_part).resolve()
        return extract(target, selector)

    return INCLUDE_RE.sub(repl, text)


def normalize_fences(text: str) -> str:
    """`rust,ignore` / `rust,no_run` -> `rust`; `wesl` -> `wgsl` (for highlighters)."""
    out = []
    for ln in text.splitlines():
        m = FENCE_RE.match(ln)
        if m and m.group(3).strip():
            info = m.group(3).strip()
            lang = info.split(",")[0].split()[0]
            if lang == "wesl":
                lang = "wgsl"
            if lang in ("console", "shell", "bash", "zsh"):
                lang = "sh"
            ln = f"{m.group(1)}{m.group(2)}{lang}"
        out.append(ln)
    return "\n".join(out) + ("\n" if text.endswith("\n") else "")


# --------------------------------------------------------------------------------------
# check
# --------------------------------------------------------------------------------------

def check_file(md: Path) -> list[str]:
    errors: list[str] = []
    text = md.read_text()
    for m in INCLUDE_RE.finditer(text):
        spec = m.group(1)
        parts = spec.split(":", 1)
        target = (md.parent / parts[0]).resolve()
        try:
            content = extract(target, parts[1] if len(parts) > 1 else None)
            if not content.strip():
                errors.append(f"{md.name}: include '{spec}' is empty")
        except IncludeError as e:
            errors.append(f"{md.name}: {e}")
    for m in re.finditer(r"!\[([^\]]*)\]\(", text):
        if re.search(r"<[A-Za-z/]", m.group(1)):
            errors.append(f"{md.name}: image caption contains '<...>', which the PDF "
                          f"drops as HTML: {m.group(1)[:60]!r} (use backticks or reword)")
    for m in IMAGE_RE.finditer(text):
        ref = m.group(1)
        if re.match(r"^[a-z]+://", ref):
            continue
        p = (md.parent / ref).resolve()
        if not p.exists():
            dot = DIAGRAMS / (p.stem + ".dot")
            if p.suffix == ".svg" and dot.exists():
                continue  # rendered at build time
            errors.append(f"{md.name}: image not found: {ref}")
    expanded = text
    try:
        expanded = expand_includes(md, text)
    except IncludeError:
        pass
    for cmd, fix in FORBIDDEN_TEX.items():
        if cmd in strip_code(text):
            errors.append(f"{md.name}: unsupported TeX command {cmd} (use {fix})")
    for ch, fix in FORBIDDEN_CHARS.items():
        if ch in expanded:
            errors.append(f"{md.name}: character U+{ord(ch):04X} has no glyph in the PDF ({fix})")
    # Unbalanced fences
    fences = sum(1 for ln in text.splitlines() if FENCE_RE.match(ln))
    if fences % 2:
        errors.append(f"{md.name}: odd number of code fences ({fences})")
    if not re.search(r"^# ", text, re.M):
        errors.append(f"{md.name}: missing H1 title")
    if len(re.findall(r"^# ", strip_code(text), re.M)) > 1:
        errors.append(f"{md.name}: more than one H1 heading")
    return errors


def strip_code(text: str) -> str:
    out, inside = [], False
    for ln in text.splitlines():
        if FENCE_RE.match(ln):
            inside = not inside
            continue
        if not inside:
            out.append(ln)
    return "\n".join(out)


def cmd_check(files: list[str]) -> int:
    paths = [Path(f).resolve() for f in files] if files else sorted(SRC.glob("*.md"))
    errors: list[str] = []
    for p in paths:
        if p.name in ("SUMMARY.md",) or p.name.startswith("part-"):
            continue
        errors += check_file(p)
    for e in errors:
        print("error:", e)
    print(f"checked {len(paths)} file(s): {len(errors)} error(s)")
    return 1 if errors else 0


# --------------------------------------------------------------------------------------
# md
# --------------------------------------------------------------------------------------

def numbered_title(entry: Entry, text: str) -> str:
    def fix(m: re.Match) -> str:
        title = m.group(1).strip()
        if entry.kind == "chapter":
            return f"# Chapter {entry.label} · {title}"
        if entry.kind == "appendix":
            return f"# Appendix {entry.label} · {title}"
        return f"# {title}"

    return re.sub(r"^# (.*)$", fix, text, count=1, flags=re.M)


def render_md(entry: Entry) -> str:
    src = SRC / entry.file
    text = src.read_text()
    text = expand_includes(src, text)
    text = normalize_fences(text)
    return numbered_title(entry, text)


def out_name(entry: Entry) -> str:
    return Path(entry.file).name


def cmd_md() -> int:
    OUT.mkdir(parents=True, exist_ok=True)
    entries = read_summary()
    n = 0
    for e in entries:
        if e.kind == "part" or not e.file or not (SRC / e.file).exists():
            if e.file and not (SRC / e.file).exists():
                print(f"warning: missing source {e.file}")
            continue
        (OUT / out_name(e)).write_text(render_md(e))
        n += 1
    print(f"wrote {n} chapter file(s) to {OUT.relative_to(ROOT)}")
    return 0


# --------------------------------------------------------------------------------------
# diagrams
# --------------------------------------------------------------------------------------

# Diagram labels are rendered with one font that has Latin, Greek and the math
# symbols the book uses (⊢ ⊗ ⊸ ⟶ ≅ λ τ ...). The PDF's SVG renderer picks a single
# font per label and falls back to an empty-box font when a glyph is missing, so a
# font with partial coverage (Helvetica) would ruin any label with a symbol in it.
# The sources keep `fontname="Helvetica"`; the substitution happens here.
DIAGRAM_FONT = "STIX Two Math"


def render_diagram(dot: Path, svg: Path) -> None:
    source = re.sub(r'fontname\s*=\s*"[^"]*"', f'fontname="{DIAGRAM_FONT}"', dot.read_text())
    res = subprocess.run(["dot", "-Tsvg"], input=source, capture_output=True, text=True)
    if res.returncode != 0:
        raise SystemExit(f"dot failed on {dot.name}:\n{res.stderr}")
    out = res.stdout.replace(f'font-family="{DIAGRAM_FONT}"', f'font-family="{DIAGRAM_FONT},serif"')
    svg.write_text(out)


def cmd_diagrams() -> int:
    IMAGES.mkdir(parents=True, exist_ok=True)
    n = 0
    for dot in sorted(DIAGRAMS.glob("*.dot")):
        svg = IMAGES / (dot.stem + ".svg")
        if (svg.exists() and svg.stat().st_mtime >= dot.stat().st_mtime
                and DIAGRAM_FONT in svg.read_text()):
            continue
        render_diagram(dot, svg)
        n += 1
    print(f"rendered {n} diagram(s)")
    return 0


# --------------------------------------------------------------------------------------
# readme
# --------------------------------------------------------------------------------------

README_START = "<!-- toc:start -->"
README_END = "<!-- toc:end -->"


def cmd_readme() -> int:
    """Rewrite the table of contents between the markers in README.md."""
    readme = ROOT / "README.md"
    lines = []
    for e in read_summary():
        if e.kind == "part":
            lines.append(f"\n**{e.title}**\n")
            lines.append("| | Chapter | Markdown | PDF |")
            lines.append("|---|---|---|---|")
            continue
        name = Path(e.file).stem
        md = f"book/chapters/{name}.md"
        pdf = f"book/chapters/{name}.pdf"
        label = {"chapter": e.label, "appendix": e.label}.get(e.kind, "")
        if e.kind == "preface":
            lines.append(f"[{e.title}]({md}) ([PDF]({pdf}))\n")
            continue
        lines.append(f"| {label} | {e.title} | [md]({md}) | [pdf]({pdf}) |")
    block = README_START + "\n" + "\n".join(lines).strip("\n") + "\n" + README_END
    text = readme.read_text()
    a, b = text.index(README_START), text.index(README_END) + len(README_END)
    readme.write_text(text[:a] + block + text[b:])
    print("updated README.md table of contents")
    return 0


# --------------------------------------------------------------------------------------
# main
# --------------------------------------------------------------------------------------

def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    c = sub.add_parser("check")
    c.add_argument("files", nargs="*")
    sub.add_parser("md")
    sub.add_parser("diagrams")
    p = sub.add_parser("pdf")
    p.add_argument("--only", help="build only the chapter whose file name starts with this")
    p.add_argument("--no-book", action="store_true", help="skip the full-book PDF")
    sub.add_parser("all")
    sub.add_parser("readme")
    a = ap.parse_args()
    if a.cmd == "readme":
        return cmd_readme()
    if a.cmd == "check":
        return cmd_check(a.files)
    if a.cmd == "md":
        return cmd_md()
    if a.cmd == "diagrams":
        return cmd_diagrams()
    if a.cmd in ("pdf", "all"):
        try:
            import pdf_build  # noqa: WPS433  (tools/pdf_build.py)
        except ImportError:
            sys.path.insert(0, str(TOOLS))
            import pdf_build
        if a.cmd == "all":
            cmd_diagrams()
            cmd_md()
            cmd_readme()
            return pdf_build.build(read_summary(), only=None, book=True)
        return pdf_build.build(read_summary(), only=a.only, book=not a.no_book)
    return 1


if __name__ == "__main__":
    sys.exit(main())
