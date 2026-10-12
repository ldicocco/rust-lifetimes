// harness: compile-fail E0716
//! In edition 2024, `impl Iterator` in a method captures the lifetime of `&self`
//! too, so the iterator keeps the temporary `Index` borrowed.

// ANCHOR: overcaptures
pub struct Index<'src> {
    src: &'src str,
}

impl<'src> Index<'src> {
    pub fn words(&self) -> impl Iterator<Item = &'src str> {
        self.src.split_whitespace()
    }
}

pub fn words_of(src: &str) -> impl Iterator<Item = &str> {
    Index { src }.words()
}
// ANCHOR_END: overcaptures
