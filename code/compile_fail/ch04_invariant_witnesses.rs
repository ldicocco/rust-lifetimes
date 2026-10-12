// harness: compile-fail "references are invariant" "pointers are invariant"
//! The invariant entries of the table: each witness tries to shrink the lifetime
//! inside `T`, and each one is rejected.

use std::cell::Cell;

// ANCHOR: witnesses
pub fn shrink_target<'long: 'short, 'short, 'v>(
    r: &'v mut &'long str,
) -> &'v mut &'short str {
    r
}

pub fn shrink_cell<'long: 'short, 'short>(c: Cell<&'long str>) -> Cell<&'short str> {
    c
}

pub fn shrink_mut_ptr<'long: 'short, 'short>(p: *mut &'long str) -> *mut &'short str {
    p
}

pub fn shrink_dyn_fn<'long: 'short, 'short>(
    f: Box<dyn Fn(&'long str)>,
) -> Box<dyn Fn(&'short str)> {
    f
}
// ANCHOR_END: witnesses
