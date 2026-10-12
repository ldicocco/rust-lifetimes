// harness: compile-fail E0597 edition=2021
//! Edition 2021: the `Ref` created in the tail expression is dropped after the
//! local `history` it borrows. Edition 2024 drops it first and accepts this code
//! (see `code/examples/ch12_guard_temporaries.rs`).

use std::cell::RefCell;

// ANCHOR: tail
pub fn entry_count() -> usize {
    let history = RefCell::new(vec![String::from("open")]);
    history.borrow().len()
}
// ANCHOR_END: tail
