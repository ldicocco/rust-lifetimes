// harness: run
//! Argument-position `impl Trait` is an anonymous type parameter, chosen by the
//! caller. Lifetimes inside it must be named (or come from `Fn` sugar), and it
//! cannot appear in a `use<..>` list.
//!
//! Run with: `cargo run --example ch08_apit`

use std::fmt::Display;

// ANCHOR: longest
// The same as `fn longest<'a, I: Iterator<Item = &'a str>>(words: I) -> ...`.
fn longest<'a>(words: impl Iterator<Item = &'a str>) -> Option<&'a str> {
    words.max_by_key(|word| word.len())
}
// ANCHOR_END: longest

// ANCHOR: sugar
// `Fn(&str)` sugar makes the elided lifetime higher-ranked (Chapter 9).
fn measure(f: impl Fn(&str) -> usize) -> usize {
    let local = String::from("created here");
    f(&local)
}
// ANCHOR_END: sugar

// ANCHOR: named
// To leave the reference's lifetime out of the captures, `T` needs a name.
fn shout<T: Display>(value: &T) -> impl Display + use<T> {
    format!("{value}!")
}
// ANCHOR_END: named

// ANCHOR: main
fn main() {
    let text = String::from("the tokenizer reads words");
    println!("{:?}", longest(text.split(' ')));
    println!("{}", measure(|s| s.len()));
    let loud;
    {
        let word = String::from("hello");
        loud = shout(&word);
    }
    println!("{loud}");
}
// ANCHOR_END: main
