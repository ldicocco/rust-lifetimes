// harness: compile-fail E0106
//! `'_` in a return type asks elision for an answer; it does not make an
//! ambiguous signature legal, and it never looks at the body.

// ANCHOR: first_word
// The body only ever returns a piece of `text`, but the signature is read
// without the body: two inputs, so elision has nothing to choose.
pub fn first_word(text: &str, separators: &str) -> &'_ str {
    text.split(|c| separators.contains(c)).next().unwrap_or("")
}
// ANCHOR_END: first_word
