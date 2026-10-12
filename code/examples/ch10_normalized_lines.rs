// harness: run
//! A lending iterator whose items borrow the iterator's own buffer: one
//! `String` is reused for every line, so the loop allocates only once.
//!
//! Run with: `cargo run --example ch10_normalized_lines`

pub trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

// ANCHOR: normalized_lines
/// The lines of a text, lowercased and with runs of spaces collapsed.
pub struct NormalizedLines<'src> {
    lines: std::str::Lines<'src>,
    buffer: String,
}

impl<'src> NormalizedLines<'src> {
    pub fn new(text: &'src str) -> Self {
        NormalizedLines {
            lines: text.lines(),
            buffer: String::new(),
        }
    }
}

impl<'src> LendingIterator for NormalizedLines<'src> {
    type Item<'a>
        = &'a str
    where
        Self: 'a;

    fn next(&mut self) -> Option<&str> {
        let line = self.lines.next()?;
        self.buffer.clear(); // overwrite the previous item
        for word in line.split_whitespace() {
            if !self.buffer.is_empty() {
                self.buffer.push(' ');
            }
            self.buffer
                .extend(word.chars().flat_map(char::to_lowercase));
        }
        Some(&self.buffer)
    }
}
// ANCHOR_END: normalized_lines

fn main() {
    // ANCHOR: main
    let config = String::from("Listen   PORT 8080\n  Log-Level  Debug\nTIMEOUT 30");
    let mut lines = NormalizedLines::new(&config);
    let mut keys = Vec::new();
    while let Some(line) = lines.next() {
        println!("{line}");
        // `line` dies at the end of this iteration; keep an owned copy.
        keys.push(line.split(' ').next().unwrap_or("").to_owned());
    }
    println!("keys: {keys:?}");
    // ANCHOR_END: main
    assert_eq!(keys, ["listen", "log-level", "timeout"]);
}
