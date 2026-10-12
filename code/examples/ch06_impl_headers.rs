// harness: run
//! Job 2 of `'_`: in an `impl` header, `'_` declares a lifetime parameter that
//! the `impl` never needs to mention. As soon as a signature or an associated
//! type must mention it, the `impl` needs a name.
//!
//! Run with: `cargo run --example ch06_impl_headers`

use std::fmt;

// ANCHOR: struct
/// The words of a borrowed text.
struct Words<'src> {
    rest: &'src str,
}
// ANCHOR_END: struct

// ANCHOR: named
// The associated type and `rest` mention the lifetime: it needs a name.
impl<'src> Iterator for Words<'src> {
    type Item = &'src str;

    fn next(&mut self) -> Option<Self::Item> {
        let trimmed = self.rest.trim_start();
        let end = trimmed.find(' ').unwrap_or(trimmed.len());
        let (word, rest) = trimmed.split_at(end);
        self.rest = rest;
        (!word.is_empty()).then_some(word)
    }
}

impl<'src> Words<'src> {
    /// What is left of the text: a borrow of the text, not of `self`.
    fn rest(&self) -> &'src str {
        self.rest
    }
}
// ANCHOR_END: named

// ANCHOR: anonymous
// Nothing here mentions the lifetime: `'_` is enough.
impl fmt::Display for Words<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Words({:?})", self.rest.trim())
    }
}

impl Words<'_> {
    fn is_done(&self) -> bool {
        self.rest.trim().is_empty()
    }
}
// ANCHOR_END: anonymous

// ANCHOR: main
fn main() {
    let text = String::from("parse the first two words only");
    let mut words = Words { rest: &text };
    let first = words.next();
    let second = words.next();
    let rest = words.rest(); // borrows `text`, so `words` can be dropped
    drop(words);
    println!("{first:?} {second:?} then {rest:?}");

    let done = Words { rest: "  " };
    println!("{done} is done: {}", done.is_done());
}
// ANCHOR_END: main
