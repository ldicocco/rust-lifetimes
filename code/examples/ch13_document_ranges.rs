// harness: run
//! The usual alternative to a self-referential struct: store positions
//! (indices) instead of references, and borrow on demand.
//!
//! Run with: `cargo run --example ch13_document_ranges`

use std::ops::Range;

// ANCHOR: document
pub struct Document {
    text: String,
    title: Range<usize>, // byte positions in `text`, not a reference
}

impl Document {
    pub fn new(text: String) -> Document {
        let end = text.find('\n').unwrap_or(text.len());
        Document {
            text,
            title: 0..end,
        }
    }

    /// Elided: `fn title<'a>(&'a self) -> &'a str`, a borrow of the document.
    pub fn title(&self) -> &str {
        &self.text[self.title.clone()]
    }

    /// Mutation is allowed, and must keep the positions valid.
    pub fn append(&mut self, more: &str) {
        self.text.push_str(more); // may reallocate: a `&str` field would dangle
    }
}
// ANCHOR_END: document

fn main() {
    // ANCHOR: main
    let mut doc = Document::new(String::from("Config\nport 8080"));
    println!("{}", doc.title());
    doc.append("\nlog-level debug");
    let moved = doc; // moving is fine: a range does not point anywhere
    println!("{} ({} bytes)", moved.title(), moved.text.len());
    // ANCHOR_END: main
}
