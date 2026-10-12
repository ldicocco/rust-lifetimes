// harness: compile-fail "is invariant over the parameter `T`"
//! The other types with interior mutability are invariant too: `RefCell`,
//! `UnsafeCell` (which all of them are built on) and `Mutex`.

use std::cell::{RefCell, UnsafeCell};
use std::sync::Mutex;

pub fn shrink_ref_cell<'long: 'short, 'short>(
    c: RefCell<&'long str>,
) -> RefCell<&'short str> {
    c
}

pub fn shrink_unsafe_cell<'long: 'short, 'short>(
    c: UnsafeCell<&'long str>,
) -> UnsafeCell<&'short str> {
    c
}

pub fn shrink_mutex<'long: 'short, 'short>(m: Mutex<&'long str>) -> Mutex<&'short str> {
    m
}
