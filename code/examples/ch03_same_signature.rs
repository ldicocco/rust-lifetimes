// harness: run
//! Elision is a rewrite of the signature: the elided and the explicit versions of
//! `first_word` have the same contract, so both coerce to the same fn pointer type
//! and fit in one array.
//!
//! Run with: `cargo run --example ch03_same_signature`

// ANCHOR: pair
// What you write:
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

// What the compiler reads:
fn first_word_explicit<'a>(text: &'a str) -> &'a str {
    text.split_whitespace().next().unwrap_or("")
}
// ANCHOR_END: pair

fn main() {
    // ANCHOR: same_type
    // An array holds values of a single type. Both functions coerce to the fn
    // pointer type `fn(&str) -> &str`, itself elided (see "Function pointers and
    // `Fn` sugar" below).
    let versions: [fn(&str) -> &str; 2] = [first_word, first_word_explicit];
    let line = String::from("parse the first word");
    for f in versions {
        println!("{}", f(&line));
    }
    // ANCHOR_END: same_type
}
