// harness: compile-fail E0597
//! A `Ref<'b, T>` holds a borrow of the cell for `'b`: the compile-time borrow
//! checker still makes sure the cell outlives every guard.

use std::cell::RefCell;

// ANCHOR: outlives
pub fn entry_count() -> usize {
    let entries;
    {
        let history = RefCell::new(vec![String::from("open")]);
        entries = history.borrow();
    }
    entries.len()
}
// ANCHOR_END: outlives
