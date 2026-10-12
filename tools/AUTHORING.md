# Authoring guide — *Rust Lifetimes*

These are the conventions the book is written with. Follow them when you fix,
extend or update a chapter, so that the book stays consistent. The outline in
`book/OUTLINE.md` defines the scope of every chapter (numbers, topics, file
names). The README explains how to build the book and run the code.

---

## 1. The reader and the tone

- The reader **writes Rust** (ownership and borrowing, structs and enums, traits
  and generics, closures, iterators, `Result`, Cargo) and has met lifetime errors
  that they could fix without being able to explain. They are **not** compiler
  experts: do not assume they know variance, higher-ranked trait bounds, GATs,
  MIR or how the borrow checker works. When the text uses such an idea, explain
  it the first time it appears in the chapter (or point back to the chapter that
  explains it).
- **Why before how.** Before a rule, say what problem it solves. After a rule,
  say what it means in plain words and show it in a program. Prefer one
  well-chosen example worked out completely over three sketched ones.
- **Every claim is a program.** The book's method: each statement about what the
  compiler accepts or rejects is backed by a file that compiles (and whose output
  is shown) or by a file that is rejected (and whose diagnostics are shown). The
  diagnostics are part of the lesson: read them with the reader, label by label.
- **The signature is the contract.** This thread runs through the whole book:
  the body of a function is checked against its signature, and every caller is
  checked against the signature and nothing else. Return to it whenever it
  explains a rule (elision, defaults, higher-ranked bounds).
- Introduce a concept with a minimal snippet, then build up. Never drop a long
  listing without a walkthrough.
- No hype, no filler, no emojis, no HTML. Voice: clear, friendly, precise, second
  person ("you"), American spelling. Use "we" for things we do together.
- Be honest about the limits of a model. "A lifetime is a set of program points"
  is the model rustc's NLL borrow checker implements; when the text simplifies,
  say so. When the Rust Reference and rustc disagree, show what rustc does (with
  a program) and say that the Reference differs.

## 2. Accuracy

1. **Every Rust snippet that claims to be working code comes from a file that
   compiles** with the pinned toolchain (Rust 1.99.0, edition 2024, see
   `code/rust-toolchain.toml`), included with `{{#include}}` (section 4). A
   fragment that is not compiled (pseudo-code, a partial signature, an
   expansion such as "the compiler reads this as ...", unstable syntax) uses the
   fence ```` ```rust,ignore ```` and the text makes clear it is not compiled.
2. **Every Rust program that must be rejected is a file in
   `code/compile_fail/`**, and the diagnostics shown in the book are included from
   its `.stderr` snapshot, which `tools/run_examples.py --bless` captures from the
   real compiler. Never type or edit a diagnostic by hand.
3. **Every Rust output shown is included from `code/output/<example>.txt`**,
   which `tools/run_examples.py --bless` captures from a real run.
4. **Verify every API, error code and message against the installed toolchain**,
   not against memory. Much of what is online describes older editions (2015,
   2018, 2021) and older compilers. The standard library docs for 1.99 and
   `rustc --explain E0xxx` are the reference.
5. **Edition-specific behavior** is stated with the edition: the book targets
   edition 2024, and shows edition 2021 behavior only where the change matters to
   the reader (for example the capture rules of `impl Trait`), with a
   compile-fail program that sets `edition=2021` (section 5).
6. **Unstable features and ongoing work** (closure lifetime binders, Polonius,
   non-lifetime binders, the next-generation trait solver) are described as such,
   with the date of the nightly or the release you checked, and the reader is
   told to check the current status. Never claim a stabilization you have not
   verified.
7. **Citations and links.** Link only to pages you have opened and checked
   (`book/src/E-further-reading.md` lists the verified ones). Do not invent deep
   links, section numbers or quotes. Quote the Reference only verbatim.
8. If something cannot be made to work, simplify the example or explain the
   limitation honestly. Never paper over it.

## 3. Where things go

```
book/src/NN-slug.md                 chapter sources (file names from OUTLINE.md)
book/src/X-slug.md                  appendices (A, B, C, ...)
book/src/SUMMARY.md                 the order of chapters, parts and appendices
book/diagrams/chNN-*.dot            Graphviz sources of the diagrams
book/images/chNN-*.svg              rendered diagrams (generated from .dot)
code/examples/chNN_*.rs             runnable single-file examples (package book-examples)
code/output/chNN_*.txt              example outputs (generated, --bless)
code/compile_fail/chNN_*.rs         programs that must be rejected
code/compile_fail/chNN_*.stderr     their diagnostics (generated, --bless)
```

`book/chapters/`, `book/Rust-Lifetimes.pdf`, the `.txt` and `.stderr` snapshots
and the README's table of contents are generated: edit the sources and rebuild
them (see the README).

- The workspace (`code/Cargo.toml`) has **no dependencies**. Use the standard
  library only. (Chapter 9 introduces a ten-line `block_on` built on
  `Waker::noop` instead of an async runtime; Chapter 11 explains it.)
- Code follows `code/rustfmt.toml` (`max_width = 88`, so that lines fit on a
  printed page). Format files with `rustfmt --edition 2024 <files>` (run from
  `code/`). Compile-fail programs are formatted too.
- Zero compiler warnings in examples (the harness fails on any). Readability
  first: descriptive names, comments that explain *why*.

## 4. Markdown format

The book is built from `book/src/*.md` into per-chapter Markdown and PDF files
plus a full-book PDF (pandoc → Typst). Write **GitHub-flavored Markdown** only.

### Chapter skeleton

```markdown
# Lifetime Elision                         <- H1 = title only; NO chapter number

Opening: 1–3 paragraphs of motivation.

**In this chapter you will learn:**       <- bold line + 4–7 bullets
- ...

## First major section                    <- H2 for sections, H3 for subsections
...
## Common confusions                       <- optional but encouraged
## Summary                                 <- bullet list of key takeaways
## Exercises                               <- 3–6 numbered exercises, each with a
                                              "*Hint:* ..." line
```

The build adds "Chapter N" automatically — do not number headings. Refer to other
chapters as "Chapter 7", to sections by name ("see *Who picks the lifetime?* in
Chapter 7"). Use H2/H3/H4 only (no H5+). A heading may contain inline code
(`` ## The anonymous lifetime `'_` ``).

**Length:** roughly 3,000–6,000 words of prose per chapter, plus code. Depth
beats breadth, but the topics listed in OUTLINE.md for the chapter must be
covered.

### Including code

Mark regions in source files with anchor comments, `// ANCHOR: name` /
`// ANCHOR_END: name`, and include them with a path **relative to the Markdown
file**:

````markdown
```rust
{{#include ../../code/examples/ch09_callee_picks.rs:with_local}}
```

```text
{{#include ../../code/output/ch09_callee_picks.txt}}
```

```rust
{{#include ../../code/compile_fail/ch09_caller_picks.rs:with_local}}
```

```text
{{#include ../../code/compile_fail/ch09_caller_picks.stderr}}
```
````

- `{{#include path}}` includes a whole file; `{{#include path:10:20}}` lines 10–20.
  Avoid line ranges for source files (they silently go stale). For a long
  `.stderr` snapshot a range is acceptable, for example to show only the first of
  two near-identical errors; say in the text that the rest is omitted.
- Anchor lines and the `// harness:` line are stripped; the region is dedented.
- Anchors may nest/overlap; names must be unique within a file.
- Mention the file a snippet comes from in the prose the first time
  (e.g. "the full program is in `code/examples/ch09_callee_picks.rs`").
- The line numbers in a diagnostic refer to the whole source file, not to the
  excerpt shown above it. That is fine; do not renumber anything.
- Say plainly before or after each program whether it compiles. A useful
  pattern: "This version compiles:" / "The compiler rejects it:".
- Check the includes with `python3 tools/build_book.py check book/src/NN-*.md`.

### Other blocks

- Fences: `rust`, `rust,ignore`, `text` (program output and diagnostics), `sh`
  (commands the reader types), `toml`.
- Callouts (GitHub alerts), used sparingly — at most ~1 per 800 words:
  `> [!NOTE]`, `> [!TIP]`, `> [!IMPORTANT]`, `> [!WARNING]`, `> [!CAUTION]`.
  Use `[!NOTE]` for edition and version notes ("Since Rust 1.89 ...").
- Figures: an image alone in its paragraph becomes a captioned figure:
  `![The borrow of v[0] is live only until its last use.](../images/ch01-nll.svg)`
  The alt text **is** the caption: write a full sentence, without `<` or `>`
  (the PDF drops them as HTML); put code in backticks.
- Tables: pipe tables, ≤ 5 columns, short cells. Tables are the best format for
  "you write / the compiler reads" comparisons.
- Mathematics: rarely needed in this book, and lifetimes never go in math (a
  `'` in TeX math is a prime). Write lifetimes, bounds and quantified bounds as
  code: `'a: 'b`, `for<'a> Fn(&'a str)`. Use plain Unicode in prose where a
  symbol helps (`⊇`, `∀`), sparingly.
- Links: only to verified URLs (see section 2). Refer to API items in `code`
  formatting.

## 5. Code conventions

### Rust examples

- File: `code/examples/chNN_<slug>.rs`.
- Line 1 is the harness directive, then an inner doc comment:

```rust
// harness: run
//! A higher-ranked bound: `f` must accept a borrow of any lifetime, including
//! one that exists only inside `with_local`.
//!
//! Run with: `cargo run --example ch09_callee_picks`
```

- **Harness directive grammar** (used by `tools/run_examples.py`):
  - `// harness: run [timeout=SECONDS]` — must build without warnings and exit
    with code 0; its standard output is compared with `code/output/<name>.txt`.
  - `// harness: build-only` — only compiled (rare).
- Examples are deterministic (no `HashMap` iteration order in the output, no
  timing), finish within a second or two, and print concise output meant to be
  shown in the book. Use `assert!`/`assert_eq!` where they make a point: an
  example is also a test.
- Keep examples focused (~15–80 lines). Several anchors in one file are fine
  when the book shows several steps of the same program.

### Compile-fail programs

- File: `code/compile_fail/chNN_<slug>.rs`, compiled on its own as a library
  (`rustc --edition 2024 --crate-type lib -A warnings`), so it needs no `main`.
  Name the function that plays the caller `caller` (or something descriptive) and
  make it `pub`, or the code may be reported as dead (harmless, but noisy).
- Line 1 lists what it must produce:

```rust
// harness: compile-fail E0597
//! A lifetime parameter of a function is chosen by the caller, so it outlives
//! the whole call: no borrow of a local variable can have it.
```

- A diagnostic without an error code ("lifetime may not live long enough",
  "implementation of `Fn` is not general enough", a lint made an error with
  `#![deny(...)]`) is checked by a quoted text instead:
  `// harness: compile-fail "may not live long enough"`. Codes and texts can be
  mixed.
- `edition=2021` compiles the file with an older edition, for the few places
  where the book shows how an edition changed the rules:
  `// harness: compile-fail E0700 edition=2021`.
- Keep them tiny (5–25 lines) so the diagnostic is short and to the point. Prefer
  one error per file; when a file must show two errors, say so in the text.
- If the diagnostic contains a note pointing into the standard library, the
  harness rewrites the local path to rustc's portable `/rustc/<hash>/...` form.
  Such notes are accurate but noisy: include the snapshot with a line range that
  stops before the note when the note adds nothing for the reader.

## 6. Diagrams

Use Graphviz for concept diagrams (0–2 per chapter where they genuinely help:
regions on a timeline, the outlives order, sets of types, who picks a
lifetime). The source goes in `book/diagrams/chNN-name.dot`;
`python3 tools/build_book.py diagrams` renders it to `book/images/chNN-name.svg`.

Shared visual language (keep it consistent across the book):

```dot
digraph G {
  graph [bgcolor="white", fontname="Helvetica", fontsize=11, pad=0.2, nodesep=0.35, ranksep=0.45];
  node  [shape=box, style="rounded,filled", fontname="Helvetica", fontsize=11,
         color="#475569", fillcolor="#f1f5f9", penwidth=1.2, margin="0.12,0.06"];
  edge  [color="#475569", fontname="Helvetica", fontsize=10, arrowsize=0.7];
  // Region / lifetime           fillcolor="#dbeafe" color="#1d4ed8"
  // Accepted / compiles         fillcolor="#dcfce7" color="#15803d"
  // Value / program point       fillcolor="#ffedd5" color="#c2410c"
  // Rejected / conflict         fillcolor="#fee2e2" color="#b91c1c"
  // Neutral / other             fillcolor="#f1f5f9" color="#475569"
}
```

Timelines (a column of program points next to columns of colored cells for
regions) are easiest as a single node with an HTML-like `<TABLE>` label: one row
per program point, `BGCOLOR` on the cells that a region covers. Keep diagrams
small (≤ ~15 nodes or rows), labels short, and use Unicode (`→`, `⊇`, `∀`) rather
than TeX.

Write `fontname="Helvetica"` as in the template: the build replaces every font
with STIX Two Math when it renders the diagrams, because the PDF's SVG renderer
uses a single font per label.

## 7. Checklist

1. Every example: zero warnings; runs according to its harness directive;
   snapshot blessed and reviewed (`python3 tools/run_examples.py --only chNN`).
2. Every compile-fail program fails with the listed codes or texts; `.stderr`
   blessed and reviewed.
3. `python3 tools/build_book.py check book/src/NN-*.md` passes.
4. The chapter follows the skeleton (learning goals, sections, summary, exercises
   with hints) and covers every topic OUTLINE.md lists for it.
5. The PDF of the chapter looks right
   (`python3 tools/build_book.py pdf --only NN --no-book`, then open
   `book/chapters/NN-*.pdf`).
6. The generated files are rebuilt: `python3 tools/build_book.py all`.
