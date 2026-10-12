// harness: run
//! The same handle with the wrong marker: `NonNull` makes it covariant in `'v`,
//! and a program that stores a dangling reference compiles.
//!
//! Run with: `cargo run --example ch13_covariant_slot`

use std::marker::PhantomData;
use std::ptr::NonNull;

// ANCHOR: covariant_slot
/// UNSOUND: covariant in `'v` (through `NonNull`), yet it writes a `&'v str`.
pub struct CovariantSlot<'s, 'v> {
    target: NonNull<&'v str>,
    _borrow: PhantomData<&'s ()>, // mentions `'s` only
}
// ANCHOR_END: covariant_slot

impl<'s, 'v> CovariantSlot<'s, 'v> {
    pub fn new(target: &'s mut &'v str) -> Self {
        CovariantSlot {
            target: NonNull::from(target),
            _borrow: PhantomData,
        }
    }

    pub fn set(&mut self, value: &'v str) {
        // SAFETY: (wrong) as for `OutSlot`, except that covariance lets a
        // caller shrink `'v` before calling `set`.
        unsafe { *self.target.as_ptr() = value }
    }
}

// ANCHOR: dangling
/// Compiles, and returns a dangling `&'static str`. Never called: calling it
/// is undefined behavior.
#[allow(dead_code)]
fn dangling() -> &'static str {
    let mut name: &'static str = "static";
    {
        let mut slot = CovariantSlot::new(&mut name);
        let temporary = String::from("temporary");
        slot.set(&temporary); // accepted: `'v` shrinks to fit `temporary`
    }
    name // points to the freed `temporary`
}
// ANCHOR_END: dangling

fn main() {
    let mut setting: &str = "unset";
    {
        let mut slot = CovariantSlot::new(&mut setting);
        slot.set("listen"); // a string literal: nothing can dangle here
    }
    println!("{setting}");
}
