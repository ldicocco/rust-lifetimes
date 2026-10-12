// harness: compile-fail E0597
//! `Cell<T>` writes through a shared reference, so it must be invariant in `T`
//! for the same reason as `&mut T`.

use std::cell::Cell;

// ANCHOR: cell
pub fn caller() {
    let label: Cell<&'static str> = Cell::new("static");
    let local = String::from("local");
    let view: &Cell<&str> = &label; // Cell<&'static str> is NOT a Cell<&'short str>
    view.set(&local);
}
// ANCHOR_END: cell
