// Typst template for "Rust Lifetimes".
//
// Used by tools/pdf_build.py, which converts each Markdown chapter to a Typst
// fragment with pandoc and then compiles either a single chapter
// (`chapter-doc`) or the whole book (`book-doc`).

#let book-title = "Rust Lifetimes"
#let book-subtitle = "A Field Guide to Regions, Elision, Variance, 'static and Higher-Ranked Bounds"
#let book-author = "Luciano Di Cocco"

#let serif = ("Charter", "Libertinus Serif", "New Computer Modern")
#let sans = ("Avenir Next", "Helvetica Neue", "Helvetica")
// The later fonts in the list supply the glyphs the first ones lack (diagnostics
// and prose use a few symbols such as → ⊆ ∀).
#let mono = ("Menlo", "DejaVu Sans Mono", "New Computer Modern Math")

#let accent = rgb("#1f5f99")
#let accent-soft = rgb("#e8f0f8")
#let ink = rgb("#1b1f24")
#let muted = rgb("#57606a")
#let code-bg = rgb("#f6f8fa")
#let code-border = rgb("#d8dee4")

// ------------------------------------------------------------------------------
// Blocks used by the converted Markdown
// ------------------------------------------------------------------------------

#let callout-styles = (
  note: (title: "Note", color: rgb("#0969da")),
  tip: (title: "Tip", color: rgb("#1a7f37")),
  important: (title: "Important", color: rgb("#8250df")),
  warning: (title: "Warning", color: rgb("#9a6700")),
  caution: (title: "Caution", color: rgb("#cf222e")),
)

#let callout(kind: "note", body) = {
  let s = callout-styles.at(kind, default: callout-styles.note)
  block(
    width: 100%,
    // Callouts are short: keep the title and the text on the same page.
    breakable: false,
    fill: s.color.lighten(93%),
    stroke: (left: 2.5pt + s.color),
    radius: (right: 3pt),
    inset: (left: 11pt, right: 10pt, top: 8pt, bottom: 9pt),
    above: 1.1em,
    below: 1.1em,
  )[
    #set par(spacing: 0.75em)
    #text(font: sans, weight: "bold", size: 0.88em, fill: s.color, upper(s.title))
    #parbreak()
    #body
  ]
}

#let learn-box(body) = block(
  width: 100%,
  breakable: true,
  fill: accent-soft,
  radius: 4pt,
  inset: (x: 13pt, y: 10pt),
  above: 1.2em,
  below: 1.4em,
)[
  #set par(spacing: 0.7em)
  #body
]

// ------------------------------------------------------------------------------
// Shared styling
// ------------------------------------------------------------------------------

#let base-style(body) = {
  set text(font: serif, size: 10.5pt, fill: ink, lang: "en", hyphenate: true)
  set par(justify: true, leading: 0.68em, spacing: 1.05em)
  show math.equation: set text(font: ("New Computer Modern Math",))
  // Inference rules are nested fractions: keep every level at full size.
  show math.frac: it => math.display(it)

  // Headings ------------------------------------------------------------------
  set heading(supplement: [Chapter])
  show heading: set text(font: sans, fill: ink)
  show heading.where(level: 2): it => {
    set text(size: 15pt, weight: "bold")
    block(above: 1.9em, below: 0.9em, sticky: true)[
      #if it.numbering != none {
        text(fill: accent)[#counter(heading).display(it.numbering)]
        h(0.6em)
      }
      #it.body
    ]
  }
  show heading.where(level: 3): it => {
    set text(size: 12pt, weight: "bold")
    block(above: 1.5em, below: 0.75em, sticky: true, it.body)
  }
  show heading.where(level: 4): it => {
    set text(size: 10.5pt, weight: "bold", style: "italic")
    block(above: 1.3em, below: 0.6em, sticky: true, it.body)
  }

  // Code ---------------------------------------------------------------------------
  show raw: set text(font: mono)
  show raw.where(block: false): it => box(
    fill: code-bg,
    inset: (x: 2.5pt),
    outset: (y: 2.5pt),
    radius: 2pt,
    text(size: 0.92em, it),
  )
  show raw.where(block: true): it => block(
    width: 100%,
    fill: code-bg,
    stroke: 0.5pt + code-border,
    radius: 3pt,
    inset: (x: 9pt, y: 8pt),
    breakable: true,
    above: 1em,
    below: 1.1em,
  )[
    #set par(justify: false, leading: 0.5em)
    #set text(size: 7.9pt)
    #it
  ]

  // Links, lists, tables, figures -------------------------------------------------
  show link: it => {
    if type(it.dest) == str { text(fill: accent, it) } else { it }
  }
  set list(indent: 0.6em, body-indent: 0.5em, spacing: 0.75em)
  set enum(indent: 0.6em, body-indent: 0.5em, spacing: 0.75em)
  set table(
    stroke: (x, y) => if y == 0 { (bottom: 0.8pt + ink) } else { (bottom: 0.4pt + code-border) },
    inset: (x: 6pt, y: 5pt),
  )
  show table: set text(size: 9.2pt)
  show table: set par(justify: false)
  show table.cell.where(y: 0): set text(weight: "bold", font: sans, size: 8.8pt)
  set figure(numbering: none, gap: 0.8em)
  show figure: set block(breakable: false, above: 1.4em, below: 1.4em)
  show figure.caption: it => {
    set text(size: 9pt, fill: muted, style: "italic")
    set par(justify: false)
    block(width: 90%, it.body)
  }
  show figure.where(kind: table): set figure.caption(position: top)
  set footnote.entry(separator: line(length: 30%, stroke: 0.5pt + muted))
  show footnote.entry: set text(size: 8.5pt)
  set quote(block: true)
  show quote.where(block: true): it => block(
    width: 100%,
    stroke: (left: 2pt + code-border),
    inset: (left: 11pt, right: 6pt, y: 3pt),
    above: 1.1em,
    below: 1.1em,
    it.body,
  )
  body
}

// Chapter / appendix title (level-1 headings) ------------------------------------
#let chapter-title-style(body) = {
  show heading.where(level: 1): it => {
    pagebreak(weak: true)
    v(1.4cm)
    if it.numbering != none {
      let n = counter(heading).get()
      let label = if it.supplement == [Appendix] {
        "Appendix " + numbering("A", n.first())
      } else {
        "Chapter " + str(n.first())
      }
      text(font: sans, size: 11pt, weight: "semibold", fill: accent, tracking: 0.12em, upper(label))
      v(0.25em)
    }
    block(text(font: sans, size: 25pt, weight: "bold", fill: ink, hyphenate: false, it.body))
    v(0.35em)
    line(length: 100%, stroke: 0.8pt + accent)
    v(1.4em)
  }
  body
}

#let chapter-numbering(..n) = {
  let nums = n.pos()
  if nums.len() <= 2 { numbering("1.1", ..nums) }
}
#let appendix-numbering(..n) = {
  let nums = n.pos()
  if nums.len() <= 2 { numbering("A.1", ..nums) }
}

// Running header showing the current chapter title.
#let running-header(short) = context {
  let here-page = here().page()
  // Do not show a header on pages that start a chapter.
  let starts = query(heading.where(level: 1)).filter(h => h.location().page() == here-page)
  if starts.len() > 0 { return }
  let before = query(selector(heading.where(level: 1)).before(here()))
  let title = if before.len() > 0 { before.last().body } else { [] }
  set text(font: sans, size: 8.5pt, fill: muted)
  grid(columns: (1fr, auto), align: (left, right), short, title)
  v(-0.3em)
  line(length: 100%, stroke: 0.4pt + code-border)
}

#let page-setup(short, body) = {
  set page(
    paper: "a4",
    margin: (top: 2.6cm, bottom: 2.4cm, inside: 2.5cm, outside: 2.3cm),
    header: running-header(short),
    footer: context {
      set text(font: sans, size: 8.5pt, fill: muted)
      align(center, counter(page).display(page.numbering))
    },
    numbering: "1",
  )
  body
}

// ------------------------------------------------------------------------------
// Book parts
// ------------------------------------------------------------------------------

#let part-page(label, title, intro) = {
  pagebreak(weak: true)
  set heading(numbering: none)
  v(4cm)
  text(font: sans, size: 12pt, weight: "semibold", fill: accent, tracking: 0.14em, upper(label))
  v(0.3em)
  // The heading itself is invisible here: it only feeds the table of contents.
  // Show only the part's own title here; the table of contents gets "Part I — ...".
  show heading.where(level: 1): it => block(text(font: sans, size: 30pt, weight: "bold", fill: ink, if title == "" { label } else { title }))
  heading(level: 1, supplement: [Part], if title == "" { label } else { label + " — " + title })
  v(0.6em)
  line(length: 40%, stroke: 1.2pt + accent)
  v(1.2em)
  set text(size: 11pt)
  intro
  pagebreak()
}

#let start-chapter(n) = {
  counter(heading).update(n - 1)
}

#let title-page(edition, date) = {
  set page(header: none, footer: none, numbering: none)
  set par(justify: false)
  v(3.2cm)
  text(font: sans, size: 13pt, weight: "semibold", fill: accent, tracking: 0.16em, upper("A field guide"))
  v(0.6em)
  text(font: sans, size: 34pt, weight: "bold", fill: ink, hyphenate: false, book-title)
  v(0.4em)
  block(width: 85%, text(font: sans, size: 16pt, fill: muted, book-subtitle))
  v(2.4cm)
  text(font: sans, size: 15pt, weight: "semibold", fill: ink, book-author)
  v(1fr)
  line(length: 100%, stroke: 1pt + accent)
  v(0.4em)
  grid(
    columns: (1fr, auto),
    text(font: sans, size: 10pt, fill: muted)[Rust 1.99 · edition 2024],
    text(font: sans, size: 10pt, fill: muted)[#edition · #date],
  )
  pagebreak()
}

// ------------------------------------------------------------------------------
// Documents
// ------------------------------------------------------------------------------

// A single chapter, appendix or the preface as a standalone PDF.
#let chapter-doc(kind: "chapter", number: 1, body) = {
  set document(title: book-title, author: book-author)
  show: base-style
  show: chapter-title-style
  show: page-setup.with(book-title)
  if kind == "chapter" {
    set heading(numbering: chapter-numbering, supplement: [Chapter])
    counter(heading).update(number - 1)
    body
  } else if kind == "appendix" {
    set heading(numbering: appendix-numbering, supplement: [Appendix])
    counter(heading).update(number - 1)
    body
  } else {
    set heading(numbering: none)
    body
  }
}

// The whole book. Front matter is passed in; chapters are included by the
// generated main file between `book-doc` and the end of the document.
#let book-doc(body) = {
  set document(title: book-title, author: book-author)
  show: base-style
  show: chapter-title-style
  show: page-setup.with(book-title)
  body
}

#let toc() = {
  pagebreak(weak: true)
  set page(numbering: "i")
  heading(level: 1, numbering: none, outlined: false)[Contents]
  show outline.entry.where(level: 1): it => {
    if it.element.supplement == [Part] {
      v(1.1em)
      text(font: sans, weight: "bold", fill: accent, it.element.body)
    } else {
      v(0.35em)
      strong(it)
    }
  }
  set outline.entry(fill: repeat(text(fill: code-border)[.], gap: 0.25em))
  outline(title: none, depth: 2, indent: auto)
}
