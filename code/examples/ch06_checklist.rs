// harness: run
//! The chapter's checklist, one function per case: write nothing, write `'_`,
//! or write a name.
//!
//! Run with: `cargo run --example ch06_checklist`

use std::str::SplitWhitespace;

// ANCHOR: once
// 1. A lifetime that appears once and is never referred to again: write nothing
//    for a reference, and `'_` for a type path.
fn words(text: &str) -> SplitWhitespace<'_> {
    text.split_whitespace()
}
// ANCHOR_END: once

// ANCHOR: tie
// 2. Two positions that must be the same lifetime: a name. The result borrows
//    `text` only, so `separators` keeps an elided lifetime of its own.
fn first_word<'t>(text: &'t str, separators: &str) -> &'t str {
    text.split(|c| separators.contains(c)).next().unwrap_or("")
}
// ANCHOR_END: tie

// ANCHOR: self_borrow
// 3. The result borrows from `self`: elide (rule 3 of elision).
struct Config {
    name: String,
}

impl Config {
    fn name(&self) -> &str {
        &self.name
    }
}
// ANCHOR_END: self_borrow

// ANCHOR: field_borrow
// 4. The result borrows from the data the struct borrows, not from `self`:
//    name the struct's parameter in the `impl` header.
struct Cursor<'src> {
    rest: &'src str,
}

impl<'src> Cursor<'src> {
    fn take(&mut self, n: usize) -> &'src str {
        let (head, tail) = self.rest.split_at(n.min(self.rest.len()));
        self.rest = tail;
        head
    }
}
// ANCHOR_END: field_borrow

// ANCHOR: object
// 5. A boxed trait object that borrows an argument: `+ '_`.
fn matcher(needle: &str) -> Box<dyn Fn(&str) -> bool + '_> {
    Box::new(move |line| line.contains(needle))
}
// ANCHOR_END: object

// ANCHOR: main
fn main() {
    let text = String::from("key=value; other=thing");
    let key;
    {
        let separators = String::from("=;");
        key = first_word(&text, &separators);
    } // `separators` is gone; `key` borrows only `text`
    println!("{key} / {} words", words(&text).count());

    let config = Config {
        name: String::from("parser"),
    };
    println!("{}", config.name());

    let mut cursor = Cursor { rest: &text };
    let (a, b) = (cursor.take(3), cursor.take(1)); // both borrow `text`
    println!("{a:?} {b:?}");

    let has_value = matcher("value");
    println!("{}", has_value(&text));
}
// ANCHOR_END: main
