// harness: run
//! Every piece of lifetime syntax in the notation table, each in code that
//! compiles. Later chapters explain the forms that this chapter only names.
//!
//! Run with: `cargo run --example ch01_notation`

use std::fmt::Display;

// ANCHOR: notation
// `'a` names a region; `&'a T` is a shared reference valid throughout it.
fn first_line<'a>(text: &'a str) -> &'a str {
    text.lines().next().unwrap_or("")
}

// `&'a mut T`: an exclusive reference, valid throughout `'a`.
fn append<'a>(buffer: &'a mut String, word: &str) -> &'a mut String {
    buffer.push_str(word);
    buffer
}

// `'long: 'short` ("'long outlives 'short"): every point of `'short` is in `'long`.
fn either<'short, 'long: 'short>(a: &'short str, b: &'long str) -> &'short str {
    if a.is_empty() { b } else { a }
}

// `T: 'a`: every borrow inside `T` is valid throughout `'a` (Chapters 7 and 8).
fn boxed<'a, T: Display + 'a>(value: T) -> Box<dyn Display + 'a> {
    Box::new(value)
}

// `'static`: the region that covers the whole run of the program (Chapter 7).
const GREETING: &'static str = "hello";

// `'_`: "some lifetime", here the one of `text` (Chapter 6).
fn words(text: &str) -> std::str::SplitWhitespace<'_> {
    text.split_whitespace()
}

// `for<'a>`: `F` must accept a borrow of every lifetime `'a` (Chapter 9).
fn apply<F>(f: F) -> usize
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let local = String::from("built inside apply");
    f(&local).len()
}

// `use<'a>`: the lifetimes an `impl Trait` return type may capture (Chapter 8).
fn lengths<'a>(text: &'a str) -> impl Iterator<Item = usize> + use<'a> {
    text.split_whitespace().map(str::len)
}
// ANCHOR_END: notation

fn main() {
    let text = String::from("regions, not durations\nsecond line");
    let mut buffer = String::from("lifetimes ");
    println!("{}", first_line(&text));
    println!("{}", append(&mut buffer, "are regions"));
    println!("{}", either("", GREETING));
    println!("{}", boxed(&text[..7]));
    println!("{}", words(&text).count());
    println!("{}", apply(|s| s.trim()));
    println!("{:?}", lengths(&text).collect::<Vec<_>>());
}
