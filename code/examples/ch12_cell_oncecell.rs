// harness: run
//! `Cell` never hands out a reference to its contents, so it needs no guard.
//! `OnceCell` hands out plain `&T` tied to `&self`, because its contents never
//! change through `&self` once they are set.
//!
//! Run with: `cargo run --example ch12_cell_oncecell`

use std::cell::{Cell, OnceCell};

// ANCHOR: document
struct Document {
    text: String,
    reads: Cell<u32>,          // a statistic updated through `&self`
    summary: OnceCell<String>, // computed on first use, then fixed
}

impl Document {
    fn new(text: &str) -> Self {
        Document {
            text: text.to_owned(),
            reads: Cell::new(0),
            summary: OnceCell::new(),
        }
    }

    /// The returned `&str` borrows `self`: no guard, no run-time count.
    fn summary(&self) -> &str {
        self.reads.set(self.reads.get() + 1);
        self.summary.get_or_init(|| {
            println!("  (computing the summary)");
            let first: Vec<&str> = self.text.split_whitespace().take(3).collect();
            first.join(" ")
        })
    }
}
// ANCHOR_END: document

// ANCHOR: cursor
/// A `Cell` can hold a borrowed value too: `Cell<&'src str>` is fine as long
/// as every slice stored in it comes from the same `'src` text.
struct Cursor<'src> {
    last_token: Cell<&'src str>,
}

impl<'src> Cursor<'src> {
    fn remember(&self, token: &'src str) {
        self.last_token.set(token);
    }
}
// ANCHOR_END: cursor

fn main() {
    // ANCHOR: use
    let doc = Document::new("lifetimes are regions of the program");
    let first = doc.summary();
    let second = doc.summary(); // both references are alive at once
    println!("{first:?} == {second:?}");
    println!("summary read {} times", doc.reads.get());
    // ANCHOR_END: use

    let source = String::from("let x = 1");
    let cursor = Cursor {
        last_token: Cell::new(""),
    };
    for token in source.split_whitespace() {
        cursor.remember(token);
    }
    println!("last token: {:?}", cursor.last_token.get());
}
