// harness: compile-fail E0515
//! The `&str` points into the vector, which is reachable only through the `Ref`
//! guard; the guard is a temporary that dies when the method returns.

use std::cell::RefCell;

pub struct History {
    entries: RefCell<Vec<String>>,
}

// ANCHOR: last
impl History {
    pub fn last(&self) -> Option<&str> {
        self.entries.borrow().last().map(|entry| entry.as_str())
    }
}
// ANCHOR_END: last
