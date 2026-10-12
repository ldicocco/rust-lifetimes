// harness: compile-fail E0515
//! Constant promotion does not apply to values with a destructor, such as a
//! `String`, or with interior mutability, such as a `Cell`.

use std::cell::Cell;

// ANCHOR: not_promoted
pub fn empty_name() -> &'static String {
    &String::new() // `String` has a destructor: not promoted
}

pub fn counter() -> &'static Cell<u32> {
    &Cell::new(0) // interior mutability: not promoted
}
// ANCHOR_END: not_promoted
