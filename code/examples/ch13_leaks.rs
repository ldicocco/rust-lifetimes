// harness: run
//! Leaking is safe: `mem::forget`, `Box::leak` and reference cycles all skip
//! destructors without any `unsafe`.
//!
//! Run with: `cargo run --example ch13_leaks`

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

// ANCHOR: guard
struct Connection {
    name: &'static str,
}

impl Drop for Connection {
    fn drop(&mut self) {
        println!("closing {}", self.name);
    }
}
// ANCHOR_END: guard

// ANCHOR: cycle
/// A node that may point to another node.
struct Node {
    _connection: Connection,
    next: RefCell<Option<Rc<Node>>>,
}
// ANCHOR_END: cycle

fn main() {
    // ANCHOR: main
    drop(Connection { name: "dropped" }); // runs `drop`
    mem::forget(Connection { name: "forgotten" }); // never runs it

    let leaked: &'static mut Connection =
        Box::leak(Box::new(Connection { name: "leaked" }));
    println!("{} lives until the end of the program", leaked.name);

    let first = Rc::new(Node {
        _connection: Connection { name: "in a cycle" },
        next: RefCell::new(None),
    });
    *first.next.borrow_mut() = Some(Rc::clone(&first)); // points to itself
    drop(first); // the count drops from 2 to 1: never 0, never freed
    println!("end of main");
    // ANCHOR_END: main
}
