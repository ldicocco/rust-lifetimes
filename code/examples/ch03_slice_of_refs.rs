// harness: run
//! `&[&str]` has two lifetimes. When the result is one of the elements, the
//! better contract ties it to the elements, not to the slice: the vector that
//! holds the words may then be a local variable.
//!
//! Run with: `cargo run --example ch03_slice_of_refs`

// ANCHOR: longest_word
fn longest_word<'w>(words: &[&'w str]) -> &'w str {
    words.iter().copied().max_by_key(|w| w.len()).unwrap_or("")
}

// Elided: fn longest_in<'t>(text: &'t str) -> &'t str
fn longest_in(text: &str) -> &str {
    let words: Vec<&str> = text.split_whitespace().collect();
    longest_word(&words) // borrows from `text`, not from the local vector
}
// ANCHOR_END: longest_word

fn main() {
    println!("{}", longest_in("the longest word wins"));
}
