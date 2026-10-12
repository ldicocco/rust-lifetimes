// harness: compile-fail E0106
//! A receiver with two references to (something containing) `Self` is ambiguous,
//! and since Rust 1.81 the compiler does not fall back to the other parameters.

// ANCHOR: two_refs
pub struct Config {
    name: String,
}

impl Config {
    pub fn name(self: &&Self, _caller: &str) -> &str {
        &self.name
    }
}
// ANCHOR_END: two_refs
