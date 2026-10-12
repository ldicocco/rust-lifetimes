# Rust Lifetimes

*A Field Guide to Regions, Elision, Variance, 'static and Higher-Ranked Bounds*

by Luciano Di Cocco

[![CI](https://github.com/ldicocco/rust-lifetimes/actions/workflows/ci.yml/badge.svg)](https://github.com/ldicocco/rust-lifetimes/actions/workflows/ci.yml)

A book about lifetimes in Rust for programmers who write Rust every day and want
to understand the rules behind the errors: what a lifetime is (a region, not a
duration), why a signature is a contract, what the compiler writes for you when
you leave a lifetime out, what `'_` and `'static` really mean, how trait objects
and `impl Trait` capture lifetimes, what `for<'a>` is for, and how all of it plays
out with generic associated types, async code, interior mutability, `unsafe` code
and API design.

Every program in the book is a file in [`code/`](code/), compiled with a pinned
**Rust 1.99.0** (edition 2024). Programs that the book says compile are run, and
their output is compared with the book; programs that the book says are rejected
are compiled, and their diagnostics are compared, verbatim, with the book.

- **Read the whole book:** [`book/Rust-Lifetimes.pdf`](book/Rust-Lifetimes.pdf)
- **Read chapter by chapter:** Markdown and PDF for every chapter in [`book/chapters/`](book/chapters/)
- **Run the code:** a Cargo workspace in [`code/`](code/)

## Contents

<!-- toc:start -->
[Preface](book/chapters/00-preface.md) ([PDF](book/chapters/00-preface.pdf))


**Part I — The Model**

| | Chapter | Markdown | PDF |
|---|---|---|---|
| 1 | Regions, Not Durations | [md](book/chapters/01-regions.md) | [pdf](book/chapters/01-regions.pdf) |
| 2 | Whose Borrow Is It? Lifetimes in Types and Methods | [md](book/chapters/02-whose-borrow.md) | [pdf](book/chapters/02-whose-borrow.pdf) |
| 3 | Lifetime Elision | [md](book/chapters/03-elision.md) | [pdf](book/chapters/03-elision.pdf) |
| 4 | Subtyping and Variance | [md](book/chapters/04-variance.md) | [pdf](book/chapters/04-variance.pdf) |
| 5 | Borrows in Practice: Reborrows, Temporaries and the Limits of NLL | [md](book/chapters/05-borrows-in-practice.md) | [pdf](book/chapters/05-borrows-in-practice.pdf) |

**Part II — Lifetimes in Signatures**

| | Chapter | Markdown | PDF |
|---|---|---|---|
| 6 | The Anonymous Lifetime '_ | [md](book/chapters/06-anonymous-lifetime.md) | [pdf](book/chapters/06-anonymous-lifetime.pdf) |
| 7 | The 'static Lifetime | [md](book/chapters/07-static.md) | [pdf](book/chapters/07-static.pdf) |
| 8 | Trait Objects, impl Trait and Captured Lifetimes | [md](book/chapters/08-trait-objects-impl-trait.md) | [pdf](book/chapters/08-trait-objects-impl-trait.pdf) |
| 9 | Higher-Ranked Trait Bounds | [md](book/chapters/09-higher-ranked.md) | [pdf](book/chapters/09-higher-ranked.pdf) |

**Part III — Going Further**

| | Chapter | Markdown | PDF |
|---|---|---|---|
| 10 | Generic Associated Types and Lending Iterators | [md](book/chapters/10-gats.md) | [pdf](book/chapters/10-gats.pdf) |
| 11 | Lifetimes in Async Code | [md](book/chapters/11-async.md) | [pdf](book/chapters/11-async.pdf) |
| 12 | Interior Mutability, Guards and Lifetimes | [md](book/chapters/12-interior-mutability.md) | [pdf](book/chapters/12-interior-mutability.pdf) |
| 13 | Lifetimes and unsafe Code | [md](book/chapters/13-unsafe.md) | [pdf](book/chapters/13-unsafe.pdf) |
| 14 | Designing APIs with Lifetimes | [md](book/chapters/14-api-design.md) | [pdf](book/chapters/14-api-design.pdf) |

**Appendices**

| | Chapter | Markdown | PDF |
|---|---|---|---|
| A | Setting Up and Verifying the Examples | [md](book/chapters/A-setup.md) | [pdf](book/chapters/A-setup.pdf) |
| B | Error Catalogue | [md](book/chapters/B-error-catalogue.md) | [pdf](book/chapters/B-error-catalogue.pdf) |
| C | Quick Reference | [md](book/chapters/C-quick-reference.md) | [pdf](book/chapters/C-quick-reference.pdf) |
| D | Glossary | [md](book/chapters/D-glossary.md) | [pdf](book/chapters/D-glossary.pdf) |
| E | Further Reading | [md](book/chapters/E-further-reading.md) | [pdf](book/chapters/E-further-reading.pdf) |
<!-- toc:end -->

## Running the code

The toolchain is pinned in `code/rust-toolchain.toml` (Rust 1.99.0 with `rustfmt`
and `rust-src`); with [rustup](https://rustup.rs) installed, it is downloaded on
first use. The code uses only the standard library.

```sh
cd code
cargo run --example ch09_callee_picks        # runnable examples: examples/chNN_*.rs
```

To verify everything the book shows (every example and its output, every program
that must be rejected and its diagnostics):

```sh
python3 tools/run_examples.py                # everything
python3 tools/run_examples.py --only ch09    # one chapter
python3 tools/run_examples.py --only ch09 --bless   # rewrite the snapshots
```

## Repository layout

```text
book/
  src/            chapter sources (Markdown with {{#include}} directives) + SUMMARY.md
  chapters/       generated: one .md and one .pdf per chapter, code inlined
  diagrams/       Graphviz sources of the diagrams; images/ has the rendered SVGs
  Rust-Lifetimes.pdf   generated: the complete book
  OUTLINE.md      the book's outline (scope of every chapter)
code/
  examples/       runnable single-file examples (chNN_*.rs)
  output/         snapshots of the examples' output, included in the book
  compile_fail/   programs that must be rejected, with their diagnostics (.stderr)
  rust-toolchain.toml, Cargo.toml, rustfmt.toml
tools/
  build_book.py   check / md / diagrams / pdf / readme / all
  run_examples.py verify every program and diagnostic (--bless to update snapshots)
  pdf_build.py, pandoc/, typst/   the PDF pipeline (pandoc -> Typst)
  AUTHORING.md    conventions used to write the book
reference/
  latex-field-guide/       the original LaTeX field guide this book grew out of
                           (rustc 1.75, edition 2021); kept for reference only
  rust-lifetimes-v0.pdf    its PDF
```

## Rebuilding the book

The chapter Markdown in `book/src/` includes code directly from the sources, so
the text always shows the code that compiles. To regenerate the per-chapter
Markdown, the PDFs and this README's table of contents:

```sh
python3 tools/build_book.py check    # validate includes, anchors and image links
python3 tools/build_book.py all      # diagrams + chapters/*.md + PDFs + README toc
```

Requirements: Python 3.11+, [pandoc](https://pandoc.org) 3.x,
[Typst](https://typst.app) 0.13+ and Graphviz (`dot`). The PDF template uses the
Charter, Avenir Next and Menlo fonts when they are installed and falls back to
Libertinus Serif, Helvetica and DejaVu Sans Mono.

## License

- **Code** (`code/`, `tools/`, and the programs, listings, captured diagnostics
  and verification script of the field guide in `reference/`): available under
  either the [MIT license](LICENSE-MIT) or the [Apache License 2.0](LICENSE-APACHE),
  at your option.
- **Book** (`book/`: text, diagrams, images and the generated Markdown and PDFs;
  and the text of the field guide in `reference/`: its LaTeX sources and PDF):
  licensed under [CC BY-NC-SA 4.0](book/LICENSE). Code excerpts in the book and
  in the field guide are also available under the code license.

This book is an independent work and is not affiliated with the Rust project.
