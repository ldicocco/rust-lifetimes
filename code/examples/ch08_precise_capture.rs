// harness: run
//! Precise capturing: `use<..>` lists the generic parameters the hidden type may
//! use. Leaving out the lifetime of `prefix` frees callers from keeping it alive.
//!
//! Run with: `cargo run --example ch08_precise_capture`

use std::fmt::Display;

// ANCHOR: use_s
// The iterator borrows `s` only; `prefix` is used during the call and forgotten.
fn after_prefix<'s>(s: &'s str, prefix: &str) -> impl Iterator<Item = char> + use<'s> {
    s.strip_prefix(prefix).unwrap_or(s).chars()
}
// ANCHOR_END: use_s

// ANCHOR: use_none
// A result that borrows nothing: capture nothing.
fn char_count(s: &str) -> impl Display + use<> {
    s.chars().count()
}
// ANCHOR_END: use_none

// ANCHOR: outlives
// An outlives bound instead of a capture list. It captures `prefix`'s lifetime
// too, but promises that the hidden type outlives `'s`.
fn after_prefix_outlives<'s>(
    s: &'s str,
    prefix: &str,
) -> impl Iterator<Item = char> + 's {
    s.strip_prefix(prefix).unwrap_or(s).chars()
}
// ANCHOR_END: outlives

// ANCHOR: labeled
struct Labeled<'a, T>(&'a str, T);

impl<T: Display> Display for Labeled<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

// The default captures `'a` and `T`, and asks nothing about how long `T` lives.
fn labeled<'a, T: Display>(label: &'a str, value: T) -> impl Display {
    Labeled(label, value)
}
// ANCHOR_END: labeled

// ANCHOR: main
fn main() {
    let text = String::from("> hello");
    let (chars, outlives, count);
    {
        let prefix = String::from("> ");
        chars = after_prefix(&text, &prefix);
        outlives = after_prefix_outlives(&text, &prefix);
        count = char_count(&prefix);
    } // `prefix` is dropped here; nothing returned borrows it
    println!("{} {} {count}", chars.collect::<String>(), outlives.count());

    let name = String::from("parser");
    println!("{}", labeled("name", &name)); // `T` is a short borrow: fine
}
// ANCHOR_END: main
