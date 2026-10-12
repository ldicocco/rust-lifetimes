// harness: compile-fail "hiding a lifetime that's elided elsewhere is confusing" "eliding a lifetime that's named elsewhere is confusing"
//! The warn-by-default lint `mismatched_lifetime_syntaxes` (Rust 1.89) flags a
//! signature that writes one lifetime in two different ways. The lint is denied
//! here so that the warnings are reported as errors; there are two.
#![deny(mismatched_lifetime_syntaxes)]

use std::str::SplitWhitespace;

// ANCHOR: hidden
pub fn words(text: &str) -> SplitWhitespace {
    text.split_whitespace()
}
// ANCHOR_END: hidden

// ANCHOR: named
pub fn trimmed<'a>(text: &'a str) -> &str {
    text.trim()
}
// ANCHOR_END: named
