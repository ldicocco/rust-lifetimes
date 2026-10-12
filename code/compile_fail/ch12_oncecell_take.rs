// harness: compile-fail E0502
//! The `&String` returned by `get_or_init` borrows the `OnceCell`; the only ways
//! to change the contents (`take`, `get_mut`) need `&mut`, which the borrow
//! checker refuses while the reference is alive.

use std::cell::OnceCell;

// ANCHOR: take
pub fn reset(summary: &mut OnceCell<String>) {
    let current = summary.get_or_init(|| String::from("draft"));
    summary.take(); // would free the `String` that `current` points to
    println!("{current}");
}
// ANCHOR_END: take
