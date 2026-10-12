"""PDF generation for *Rust Lifetimes* (called from tools/build_book.py).

Pipeline per chapter:
    book/src/NN-x.md --(expand includes)--> book/.build/NN-x.md
                     --(pandoc + tools/pandoc/filters.lua)--> book/.build/NN-x.typ
                     --(typst + tools/typst/book.typ)--> book/chapters/NN-x.pdf
and the whole book is assembled from the same Typst fragments into
book/Rust-Lifetimes.pdf.
"""

from __future__ import annotations

import datetime
import re
import shutil
import subprocess
import sys
from pathlib import Path

import build_book as bb

BUILD = bb.BOOK / ".build"
TEMPLATE_IMPORT = '#import "../../tools/typst/book.typ": *'
FILTER = bb.TOOLS / "pandoc" / "filters.lua"
FONT_PATH = bb.TOOLS / "typst" / "fonts"
BOOK_PDF = bb.BOOK / "Rust-Lifetimes.pdf"


def run(cmd: list[str], cwd: Path | None = None) -> None:
    res = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)
    if res.returncode != 0:
        sys.stderr.write(res.stdout + res.stderr)
        raise SystemExit(f"command failed: {' '.join(cmd)}")
    if res.stderr.strip():
        # typst/pandoc warnings: show them, they usually point at real problems
        sys.stderr.write(res.stderr)


def to_typst(src_md: Path, stem: str) -> Path:
    """Expand includes and convert one Markdown source into a Typst fragment."""
    text = bb.expand_includes(src_md, src_md.read_text())
    text = bb.normalize_fences(text)
    md = BUILD / f"{stem}.md"
    md.write_text(text)
    typ = BUILD / f"{stem}.typ"
    run(
        [
            "pandoc",
            "-f", "gfm",
            "-t", "typst",
            "--wrap=preserve",
            "--lua-filter", str(FILTER),
            "-M", f"idprefix={stem}-",
            md.name,
            "-o", typ.name,
        ],
        cwd=BUILD,
    )
    # Fragments are `#include`d, and included files do not see the importer's
    # definitions: give each fragment access to the template's blocks.
    body = fix_math(fix_table_alignment(typ.read_text()))
    typ.write_text('#import "../../tools/typst/book.typ": callout, learn-box\n' + body)
    return typ


def fix_table_alignment(typst: str) -> str:
    """Vertically center table cells (pandoc only sets horizontal alignment)."""

    def fix(m):
        items = [x.strip() for x in m.group(1).split(",") if x.strip()]
        items = [("left" if x == "auto" else x) + " + horizon" for x in items]
        return "align: (" + ", ".join(items) + ",)"

    return re.sub(r"align: \(((?:auto|left|center|right|,)+)\)", fix, typst)


def fix_math(typst: str) -> str:
    """Restore the linear-logic "with" connective.

    `&` is an alignment point in Typst math, so the Lua filter replaces `\\&` in TeX
    math with a fullwidth ampersand, which becomes the `amp` symbol here.
    """
    amp = "\uff06"
    # A lone connective ($\mathbin{\&}$) must stay inline: in Typst, spaces just
    # inside both dollars would turn it into display math.
    typst = re.sub(r"\$\s*" + amp + r"\s*\$", "$amp$", typst)
    typst = re.sub(r"(?<=\$)" + amp, "amp med ", typst)
    typst = re.sub(amp + r"(?=\$)", " med amp", typst)
    return typst.replace(amp, " med amp med ")


def compile_typst(main: Path, pdf: Path) -> None:
    cmd = ["typst", "compile", "--root", str(bb.ROOT)]
    if FONT_PATH.is_dir():
        cmd += ["--font-path", str(FONT_PATH)]
    run(cmd + [str(main), str(pdf)])


def split_part_title(title: str) -> tuple[str, str]:
    for sep in (" — ", " - ", ": "):
        if sep in title:
            a, b = title.split(sep, 1)
            return a.strip(), b.strip()
    return title.strip(), ""


def typst_str(s: str) -> str:
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'


def chapter_main(entry: bb.Entry, typ: Path) -> Path:
    kind = entry.kind if entry.kind in ("chapter", "appendix") else "front"
    number = 1
    if entry.kind == "chapter":
        number = int(entry.label)
    elif entry.kind == "appendix":
        number = ord(entry.label) - ord("A") + 1
    main = BUILD / f"{typ.stem}.main.typ"
    main.write_text(
        f"{TEMPLATE_IMPORT}\n"
        f'#show: chapter-doc.with(kind: "{kind}", number: {number})\n'
        f'#include "{typ.name}"\n'
    )
    return main


def about_page(date: str) -> str:
    return f"""
#page(header: none, footer: none)[
#set text(size: 9pt, fill: muted)
#set par(justify: false)
#v(1fr)
*#book-title* --- #book-subtitle

Copyright © 2026 #book-author

First edition, {date}. This book targets *Rust 1.99* (edition 2024). The code
uses only the standard library.

Every program shown in this book is part of the accompanying `code/` directory, a
Cargo workspace. Code excerpts are included verbatim from those files when the
book is built. Program outputs and compiler diagnostics were captured from real
runs of the pinned compiler and are checked again every time the examples are
verified: a program that the book says compiles must compile without warnings
and print exactly the output shown, and a program that the book says is rejected
must fail with exactly the diagnostics shown.

The text of this book and its diagrams are licensed under the Creative Commons
Attribution-NonCommercial-ShareAlike 4.0 International License (CC BY-NC-SA 4.0).
The code, including the excerpts shown in the book, is available under the MIT
license or the Apache License, Version 2.0, at your option.

Rust is a trademark of the Rust Foundation. This book is an independent work and
is not affiliated with or endorsed by the Rust project.
#v(1cm)
]
"""


def book_main(entries: list[bb.Entry], typs: dict[str, Path]) -> Path:
    date = datetime.date.today().strftime("%B %Y")
    lines = [
        TEMPLATE_IMPORT,
        "#show: book-doc",
        f'#title-page("First edition", "{date}")',
        about_page(date),
        "#toc()",
        "#set page(numbering: \"i\")",
    ]
    main_matter_started = False
    mode = None
    for e in entries:
        if e.kind == "part":
            if not main_matter_started:
                lines.append('#set page(numbering: "1")')
                lines.append("#counter(page).update(1)")
                main_matter_started = True
            label, title = split_part_title(e.title)
            intro = "[]"
            if e.part_intro and (bb.SRC / e.part_intro).exists():
                stem = Path(e.part_intro).stem
                if stem in typs:
                    intro = f'include "{typs[stem].name}"'
            lines.append(f"#part-page({typst_str(label)}, {typst_str(title)}, {intro})")
            continue
        stem = Path(e.file).stem
        if stem not in typs:
            lines.append(f"// missing: {e.file}")
            continue
        if e.kind == "preface":
            if mode != "front":
                lines.append("#set heading(numbering: none)")
                mode = "front"
        elif e.kind == "chapter":
            if mode != "chapter":
                lines.append("#set heading(numbering: chapter-numbering, supplement: [Chapter])")
                mode = "chapter"
            lines.append(f"#start-chapter({int(e.label)})")
        elif e.kind == "appendix":
            if mode != "appendix":
                lines.append("#set heading(numbering: appendix-numbering, supplement: [Appendix])")
                mode = "appendix"
            lines.append(f"#start-chapter({ord(e.label) - ord('A') + 1})")
        lines.append(f'#include "{typs[stem].name}"')
    main = BUILD / "book.main.typ"
    main.write_text("\n".join(lines) + "\n")
    return main


def build(entries: list[bb.Entry], only: str | None = None, book: bool = True) -> int:
    for tool in ("pandoc", "typst"):
        if not shutil.which(tool):
            raise SystemExit(f"{tool} is required")
    bb.cmd_diagrams()
    BUILD.mkdir(parents=True, exist_ok=True)
    bb.OUT.mkdir(parents=True, exist_ok=True)

    typs: dict[str, Path] = {}
    # Part introductions (optional files book/src/part-*.md).
    for e in entries:
        if e.kind == "part" and e.part_intro and (bb.SRC / e.part_intro).exists():
            stem = Path(e.part_intro).stem
            typs[stem] = to_typst(bb.SRC / e.part_intro, stem)

    built = 0
    for e in entries:
        if e.kind == "part" or not e.file:
            continue
        src = bb.SRC / e.file
        if not src.exists():
            print(f"warning: missing {e.file}")
            continue
        stem = Path(e.file).stem
        selected = only is None or stem.startswith(only)
        if not book and not selected:
            # A single-chapter build must not depend on other chapters, which
            # may be work in progress.
            continue
        typs[stem] = to_typst(src, stem)
        if selected:
            pdf = bb.OUT / f"{stem}.pdf"
            compile_typst(chapter_main(e, typs[stem]), pdf)
            built += 1
            print(f"pdf: {pdf.relative_to(bb.ROOT)}")
    if book:
        compile_typst(book_main(entries, typs), BOOK_PDF)
        print(f"pdf: {BOOK_PDF.relative_to(bb.ROOT)}")
    print(f"built {built} chapter PDF(s)")
    return 0
