// harness: compile-fail E0658
//! An elided or `'_` lifetime inside argument-position `impl Trait` is unstable
//! as of Rust 1.99.

// ANCHOR: anonymous
pub fn longest(words: impl Iterator<Item = &'_ str>) -> usize {
    words.map(str::len).max().unwrap_or(0)
}
// ANCHOR_END: anonymous
