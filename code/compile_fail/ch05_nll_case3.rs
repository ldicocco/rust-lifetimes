// harness: compile-fail E0502
//! NLL problem case #3: a borrow that is returned on one path is treated as live
//! on every path. Rejected by stable Rust 1.99; accepted by nightly 2026-10-08,
//! whose borrow checker (Polonius Alpha) tracks the two paths separately.

use std::collections::HashMap;

// ANCHOR: cached
pub fn cached<'m>(cache: &'m mut HashMap<String, String>, word: &str) -> &'m String {
    if let Some(found) = cache.get(word) {
        return found;
    }
    cache.insert(word.to_owned(), word.to_uppercase());
    &cache[word]
}
// ANCHOR_END: cached
