// harness: run
//! Conflicting `RefCell` borrows compile and panic at run time. This example
//! catches the panics to show their messages; real code would use
//! `try_borrow_mut` or, better, shorter-lived guards.
//!
//! Run with: `cargo run --example ch12_double_borrow`

use std::cell::RefCell;
use std::panic::{self, AssertUnwindSafe};

/// Runs `f`, which must panic, and returns the panic message.
fn panic_message(f: impl FnOnce()) -> String {
    let payload = panic::catch_unwind(AssertUnwindSafe(f)).expect_err("must panic");
    match payload.downcast_ref::<String>() {
        Some(message) => message.clone(),
        None => payload.downcast_ref::<&str>().unwrap_or(&"?").to_string(),
    }
}

fn main() {
    panic::set_hook(Box::new(|_| {})); // keep the default report off stderr
    let words = RefCell::new(vec![String::from("alpha")]);

    // ANCHOR: panics
    let message = panic_message(|| {
        let reading = words.borrow();
        words.borrow_mut().push(String::from("beta")); // compiles; panics
        println!("never printed: {}", reading.len());
    });
    println!("borrow_mut while a Ref is alive: {message}");

    let message = panic_message(|| {
        let mut writing = words.borrow_mut();
        writing.push(format!("{} words", words.borrow().len())); // panics
    });
    println!("borrow while a RefMut is alive: {message}");
    // ANCHOR_END: panics

    let _ = panic::take_hook(); // restore the default hook

    // ANCHOR: after
    // Unwinding dropped the guards, so the cell is usable again.
    words.borrow_mut().push(String::from("gamma"));
    println!("after the panics: {:?}", words.borrow());
    // ANCHOR_END: after
}
