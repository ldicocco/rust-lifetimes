// harness: run
//! In edition 2024, return-position `impl Trait` captures every generic
//! parameter in scope, including elided lifetimes: the hidden type of
//! `chars_of` may borrow `s` without any annotation.
//!
//! Run with: `cargo run --example ch08_rpit_captures`

// ANCHOR: chars_of
fn chars_of(s: &str) -> impl Iterator<Item = char> {
    s.chars() // the hidden type is `Chars<'_>`, which borrows `s`
}
// ANCHOR_END: chars_of

// ANCHOR: explicit
// The same contract with the captures written out.
fn chars_of_explicit<'s>(s: &'s str) -> impl Iterator<Item = char> + use<'s> {
    s.chars()
}
// ANCHOR_END: explicit

// ANCHOR: both
// The hidden type borrows from both inputs, and both lifetimes are captured.
fn both<'a, 'b>(x: &'a str, y: &'b str) -> impl Iterator<Item = char> {
    x.chars().chain(y.chars())
}
// ANCHOR_END: both

// ANCHOR: generic
// Type parameters have always been captured, in every edition.
fn repeated<T: Clone>(item: T, times: usize) -> impl Iterator<Item = T> {
    std::iter::repeat_n(item, times)
}
// ANCHOR_END: generic

fn main() {
    let text = String::from("abc");
    let upper: String = chars_of(&text).map(|c| c.to_ascii_uppercase()).collect();
    let count = chars_of_explicit(&text).count();
    let echo: Vec<&str> = repeated("ok", 2).collect();
    let joined: String = both(&text, "!").collect();
    println!("{upper} {count} {echo:?} {joined}");
}
