// harness: run
//! A parser that borrows its input. `next_word` returns words that borrow from
//! the text (`'src`), not from the `&mut self` borrow of the parser.
//!
//! Run with: `cargo run --example ch02_parser`

use std::fmt;

// ANCHOR: struct
struct Parser<'src> {
    input: &'src str,
    pos: usize,
}
// ANCHOR_END: struct

// ANCHOR: impl
impl<'src> Parser<'src> {
    fn new(input: &'src str) -> Self {
        Parser { input, pos: 0 }
    }

    /// The result borrows from the text, so earlier results stay usable
    /// across later calls.
    fn next_word(&mut self) -> Option<&'src str> {
        let input = self.input; // a copy of the field: a `&'src str`
        let rest = &input[self.pos..];
        let trimmed = rest.trim_start();
        if trimmed.is_empty() {
            return None;
        }
        let start = self.pos + (rest.len() - trimmed.len());
        let end = trimmed
            .find(char::is_whitespace)
            .map_or(input.len(), |i| start + i);
        self.pos = end;
        Some(&input[start..end])
    }
}
// ANCHOR_END: impl

// ANCHOR: display
// `'_`: "the parser's lifetime parameter, whatever it is" (Chapter 6).
impl fmt::Display for Parser<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parser@{} over {:?}", self.pos, self.input)
    }
}
// ANCHOR_END: display

// ANCHOR: all_words
fn all_words(text: &str) -> Vec<&str> {
    let mut parser = Parser::new(text);
    let mut words = Vec::new();
    while let Some(word) = parser.next_word() {
        words.push(word);
    }
    words // `parser` is dropped here; the words borrow `text`, not `parser`
}
// ANCHOR_END: all_words

// ANCHOR: main
fn main() {
    let text = String::from("lifetimes are regions");
    let mut parser = Parser::new(&text);
    let first = parser.next_word();
    let second = parser.next_word(); // fine: `first` does not keep `parser` borrowed
    println!("{first:?} {second:?} / {parser}");
    println!("{:?}", all_words(&text));
}
// ANCHOR_END: main
