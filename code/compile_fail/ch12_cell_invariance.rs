// harness: compile-fail E0597
//! `Cell<T>` is invariant in `T`: a `&Cell<&'static str>` cannot be used as a
//! `&Cell<&'a str>` for a shorter `'a`, so `'a` is forced to be `'static`.

use std::cell::Cell;

// ANCHOR: invariance
fn remember<'a>(slot: &Cell<&'a str>, token: &'a str) {
    slot.set(token);
}

pub fn caller(slot: &Cell<&'static str>) {
    let local = String::from("short-lived");
    remember(slot, &local); // would leave a dangling `&str` in `slot`
}
// ANCHOR_END: invariance
