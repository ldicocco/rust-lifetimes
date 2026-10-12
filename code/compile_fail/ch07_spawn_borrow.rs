// harness: compile-fail E0373
//! `thread::spawn` requires a `'static` closure, so the closure cannot borrow a
//! local variable.

use std::thread;

// ANCHOR: spawn_borrow
pub fn caller() {
    let words = vec![String::from("alpha"), String::from("beta")];
    let handle = thread::spawn(|| words.len()); // borrows `words`
    println!("{}", handle.join().unwrap());
}
// ANCHOR_END: spawn_borrow
