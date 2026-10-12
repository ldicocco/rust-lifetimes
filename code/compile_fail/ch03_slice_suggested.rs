// harness: compile-fail E0515
//! The compiler's suggestion for `longest_word` ties the result to the slice as
//! well, so a caller can no longer pass a local vector and return the result.

// ANCHOR: suggested
pub fn longest_word<'a>(words: &'a [&'a str]) -> &'a str {
    words.iter().copied().max_by_key(|w| w.len()).unwrap_or("")
}

pub fn longest_in(text: &str) -> &str {
    let words: Vec<&str> = text.split_whitespace().collect();
    longest_word(&words)
}
// ANCHOR_END: suggested
