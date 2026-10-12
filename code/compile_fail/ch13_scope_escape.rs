// harness: compile-fail "lifetime may not live long enough"
//! The `'scope` lifetime is chosen inside `thread::scope`, so nothing that
//! carries it can be returned from the closure.

use std::thread;

// ANCHOR: escape
pub fn escape() -> usize {
    let words = vec!["listen", "port"];
    let handle = thread::scope(|s| s.spawn(|| words.len()));
    handle.join().unwrap()
}
// ANCHOR_END: escape
