// harness: compile-fail E0700 edition=2021
//! Edition 2021: return-position `impl Trait` captured type parameters but only
//! the lifetimes named in its bounds. `chars_of` is rejected; the two other
//! functions show the fixes, which 2021 accepts.

// ANCHOR: chars_of
pub fn chars_of(s: &str) -> impl Iterator<Item = char> {
    s.chars()
}
// ANCHOR_END: chars_of

// ANCHOR: fixes
// The outlives trick: mentioning `'_` in a bound made it captured.
pub fn chars_outlives(s: &str) -> impl Iterator<Item = char> + '_ {
    s.chars()
}

// Precise capturing (Rust 1.82), which works in every edition.
pub fn chars_use(s: &str) -> impl Iterator<Item = char> + use<'_> {
    s.chars()
}
// ANCHOR_END: fixes
