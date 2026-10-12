// harness: compile-fail "must mention all type parameters in scope"
//! A `use<..>` list may leave out lifetimes, but not type parameters (as of
//! Rust 1.99).

use std::fmt::Display;

// ANCHOR: types
pub fn tagged<T: Display>(tag: T, text: &str) -> impl Display + use<'_> {
    format!("[{tag}] {text}")
}
// ANCHOR_END: types
