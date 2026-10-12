// harness: compile-fail E0597
//! The invariant marker makes the compiler reject storing a short-lived string
//! into a slot that holds a `&'static str`.

use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct OutSlot<'s, 'v> {
    target: NonNull<&'v str>,
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
        // SAFETY: see `ch13_out_slot.rs`.
        unsafe { *self.target.as_ptr() = value }
    }
}

// ANCHOR: dangling
pub fn dangling() -> &'static str {
    let mut name: &'static str = "static";
    {
        let mut slot = OutSlot::new(&mut name);
        let temporary = String::from("temporary");
        slot.set(&temporary);
    }
    name
}
// ANCHOR_END: dangling
