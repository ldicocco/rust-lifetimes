// harness: compile-fail E0106
//! One parameter, but two lifetimes in it: the slice borrow and the borrows of
//! its elements. Rule 2 needs exactly one, so the output is ambiguous.

// ANCHOR: slice_of_refs
pub fn longest_word(words: &[&str]) -> &str {
    words.iter().copied().max_by_key(|w| w.len()).unwrap_or("")
}
// ANCHOR_END: slice_of_refs
