// harness: compile-fail "lifetime may not live long enough"
//! A struct with a `Cell<&'a str>` field is invariant in `'a`, and the diagnostic
//! says so.

use std::cell::Cell;

// ANCHOR: slot
pub struct Slot<'a> {
    current: Cell<&'a str>,
}

pub fn shrink_slot<'long: 'short, 'short>(s: Slot<'long>) -> Slot<'short> {
    s
}
// ANCHOR_END: slot
