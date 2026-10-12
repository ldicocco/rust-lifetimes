// harness: run
//! `RefCell` moves the "shared XOR mutable" check from compile time to run time:
//! `borrow` and `borrow_mut` take `&self` and return guards (`Ref`, `RefMut`) that
//! keep a count in the cell until they are dropped.
//!
//! Run with: `cargo run --example ch12_refcell_guard`

use std::cell::RefCell;

// ANCHOR: history
/// A command history that can be appended to through a shared reference.
struct History {
    entries: RefCell<Vec<String>>,
}

impl History {
    fn new() -> Self {
        History {
            entries: RefCell::new(Vec::new()),
        }
    }

    fn record(&self, entry: &str) {
        self.entries.borrow_mut().push(entry.to_owned()); // `&self` is enough
    }

    fn len(&self) -> usize {
        self.entries.borrow().len()
    }
}
// ANCHOR_END: history

fn main() {
    // ANCHOR: shared
    let history = History::new();
    let editor = &history;
    let autosave = &history; // two shared references to the same history
    editor.record("open notes.txt");
    autosave.record("save notes.txt");
    println!("{} entries", history.len());
    // ANCHOR_END: shared

    // ANCHOR: dynamic
    let entries = history.entries.borrow(); // a `Ref`: a shared borrow at run time
    match history.entries.try_borrow_mut() {
        Ok(_) => println!("with a Ref alive: got a RefMut"),
        Err(error) => println!("with a Ref alive: {error}"),
    }
    println!("still readable: {:?}", entries.first());
    drop(entries); // the run-time borrow ends here, not at the closing brace
    match history.entries.try_borrow_mut() {
        Ok(mut entries) => {
            entries.push(String::from("close notes.txt"));
            println!("after drop: got a RefMut");
        }
        Err(error) => println!("after drop: {error}"),
    }
    // ANCHOR_END: dynamic
    assert_eq!(history.len(), 3);
}
