// harness: run
//! Iterators that return references: `impl Iterator<Item = &str>` from a method
//! (edition 2024 captures the lifetime of `&self`), a named iterator type, and
//! `use<'src>` for items that borrow from the source rather than from `self`.
//!
//! Run with: `cargo run --example ch14_iterators`

/// A document that owns its text.
struct Document {
    body: String,
}

impl Document {
    // ANCHOR: impl_iter
    /// The items borrow from `self`; in edition 2024 the opaque type captures
    /// the elided lifetime of `&self`, so no `+ '_` is needed.
    fn words(&self) -> impl Iterator<Item = &str> {
        self.body.split_whitespace()
    }

    fn long_words(&self, min_len: usize) -> impl Iterator<Item = &str> {
        self.words().filter(move |word| word.len() >= min_len)
    }
    // ANCHOR_END: impl_iter

    // ANCHOR: named_method
    /// Returns a named type, which callers can store in a field.
    fn lines(&self) -> BodyLines<'_> {
        BodyLines {
            inner: self.body.lines(),
        }
    }
    // ANCHOR_END: named_method
}

// ANCHOR: named
/// The non-blank lines of a document, trimmed.
struct BodyLines<'doc> {
    inner: std::str::Lines<'doc>,
}

impl<'doc> Iterator for BodyLines<'doc> {
    type Item = &'doc str;

    fn next(&mut self) -> Option<&'doc str> {
        self.inner
            .by_ref()
            .map(str::trim)
            .find(|line| !line.is_empty())
    }
}

/// A reader that keeps its position in a document: possible because the
/// iterator type has a name.
struct Reader<'doc> {
    lines: BodyLines<'doc>,
    line_no: usize,
}
// ANCHOR_END: named

// ANCHOR: use_src
/// A short-lived view of some source text.
struct Index<'src> {
    src: &'src str,
}

impl<'src> Index<'src> {
    /// The items borrow from the source, not from the index: `use<'src>` says
    /// that the iterator captures only `'src` (Chapter 8).
    fn words(&self) -> impl Iterator<Item = &'src str> + use<'src> {
        self.src.split_whitespace()
    }
}

/// The `Index` is a temporary, dropped at the end of the function; the
/// iterator survives it because it does not borrow it.
fn words_of(src: &str) -> impl Iterator<Item = &str> {
    Index { src }.words()
}
// ANCHOR_END: use_src

fn main() {
    // ANCHOR: main
    let doc = Document {
        body: String::from("  borrow checkers\n\n  check borrows \n"),
    };
    println!("{:?}", doc.words().collect::<Vec<_>>());
    println!("{:?}", doc.long_words(7).collect::<Vec<_>>());

    let mut reader = Reader {
        lines: doc.lines(),
        line_no: 0,
    };
    while let Some(line) = reader.lines.next() {
        reader.line_no += 1;
        println!("{}: {line}", reader.line_no);
    }

    println!("{:?}", words_of("one two three").last());
    // ANCHOR_END: main
}
