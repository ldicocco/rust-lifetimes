// harness: compile-fail "lifetime may not live long enough"
//! Following the help of `ch13_scope_escape.rs` does not help: the handle
//! still carries `'scope`.

use std::thread;

// ANCHOR: escape
pub fn escape() -> usize {
    let words = vec!["listen", "port"];
    let handle = thread::scope(|s| s.spawn(move || words.len()));
    handle.join().unwrap()
}
// ANCHOR_END: escape
