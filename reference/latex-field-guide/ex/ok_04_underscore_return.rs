#![warn(elided_lifetimes_in_paths)]
use std::str::SplitWhitespace;

// Explicit: the hidden lifetime parameter of SplitWhitespace is visible.
fn words(s: &str) -> SplitWhitespace<'_> {
    s.split_whitespace()
}

// Legal, but the borrow is invisible in the signature (the lint fires).
fn words_hidden(s: &str) -> SplitWhitespace {
    s.split_whitespace()
}

fn main() {
    let t = String::from("one two three");
    println!("{}", words(&t).count() + words_hidden(&t).count());
}
