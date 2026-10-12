// harness: run
//! Three ways to give callers access to data inside a `RefCell`: return a guard
//! narrowed with `Ref::map`/`Ref::filter_map`, lend the data to a closure, or
//! return an owned copy.
//!
//! Run with: `cargo run --example ch12_ref_map`

use std::cell::{Ref, RefCell};

struct History {
    entries: RefCell<Vec<String>>,
}

// ANCHOR: guards
impl History {
    /// The whole list, as a guard that derefs to a slice.
    fn entries(&self) -> Ref<'_, [String]> {
        Ref::map(self.entries.borrow(), |entries| entries.as_slice())
    }

    /// The last entry, if any: `filter_map` gives the guard back on `None`.
    fn last(&self) -> Option<Ref<'_, str>> {
        Ref::filter_map(self.entries.borrow(), |entries| {
            entries.last().map(|entry| entry.as_str())
        })
        .ok()
    }
}
// ANCHOR_END: guards

// ANCHOR: closure
impl History {
    /// Lends the last entry to `f`; the reference cannot escape the call.
    fn with_last<R>(&self, f: impl FnOnce(Option<&str>) -> R) -> R {
        let entries = self.entries.borrow();
        f(entries.last().map(|entry| entry.as_str()))
    }

    /// Or give up on borrowing: an owned copy has no guard at all.
    fn last_owned(&self) -> Option<String> {
        self.entries.borrow().last().cloned()
    }
}
// ANCHOR_END: closure

fn main() {
    let history = History {
        entries: RefCell::new(vec![String::from("open"), String::from("edit")]),
    };

    // ANCHOR: use
    if let Some(last) = history.last() {
        println!("last: {}", &*last); // `last` is a `Ref<'_, str>`
    } // the guard is dropped here, so recording below is fine
    history.entries.borrow_mut().push(String::from("save"));

    println!("all: {:?}", &*history.entries());
    let shouting = history.with_last(|last| last.map(str::to_uppercase));
    println!("shouting: {shouting:?}");
    let owned = history.last_owned();
    history.entries.borrow_mut().clear(); // no guard is alive
    println!("owned: {owned:?}, left: {}", history.entries().len());
    // ANCHOR_END: use
}
