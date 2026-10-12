// harness: run
//! A write handle built on `NonNull`, which is covariant. Writing through it
//! is only sound if the type is invariant in the lifetime of what it stores,
//! so a `PhantomData<&'s mut &'v str>` marker makes it so.
//!
//! Run with: `cargo run --example ch13_out_slot`

use std::marker::PhantomData;
use std::ptr::NonNull;

// ANCHOR: out_slot
/// A handle that can overwrite a `&'v str` owned by someone else, for `'s`.
/// (In safe code you would store the `&'s mut &'v str` itself; the raw pointer
/// stands for one that came from C, or from a node of a linked structure.)
pub struct OutSlot<'s, 'v> {
    target: NonNull<&'v str>,
    /// Covariant in `'s`, invariant in `'v`, like `&'s mut &'v str`.
    _borrow: PhantomData<&'s mut &'v str>,
}

impl<'s, 'v> OutSlot<'s, 'v> {
    pub fn new(target: &'s mut &'v str) -> Self {
        OutSlot {
            target: NonNull::from(target),
            _borrow: PhantomData,
        }
    }

    pub fn set(&mut self, value: &'v str) {
        // SAFETY: `target` came from a `&'s mut &'v str` that this handle
        // keeps borrowed for `'s`, so the slot is valid and nobody else can
        // access it. `value` lives for `'v`, the lifetime the slot's owner
        // expects; invariance in `'v` stops anyone from shrinking `'v` first.
        unsafe { *self.target.as_ptr() = value }
    }
}
// ANCHOR_END: out_slot

fn main() {
    // ANCHOR: main
    let mut setting: &str = "unset";
    {
        let mut slot = OutSlot::new(&mut setting);
        slot.set("listen");
    }
    println!("{setting}");
    // ANCHOR_END: main
}
