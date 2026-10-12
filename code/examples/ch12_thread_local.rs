// harness: run
//! `thread_local!` values are reached through `LocalKey::with` and friends, which
//! lend a reference to a closure. Each thread has its own copy.
//!
//! Run with: `cargo run --example ch12_thread_local`

use std::cell::{Cell, RefCell};
use std::thread;

// ANCHOR: declare
thread_local! {
    static LOG: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    static DEPTH: Cell<u32> = const { Cell::new(0) };
}
// ANCHOR_END: declare

// ANCHOR: access
fn log(line: &str) {
    LOG.with(|log| log.borrow_mut().push(line.to_owned()));
}

fn log_len() -> usize {
    LOG.with_borrow(|log| log.len()) // the result is a `usize`: nothing borrowed
}

fn first_entry() -> Option<String> {
    LOG.with_borrow(|log| log.first().cloned()) // an owned copy leaves the closure
}
// ANCHOR_END: access

fn main() {
    // ANCHOR: threads
    log("main: start");
    DEPTH.set(DEPTH.get() + 1);
    thread::scope(|s| {
        s.spawn(|| {
            log("worker: start");
            println!("worker: log length {}, depth {}", log_len(), DEPTH.get());
        });
    });
    log("main: done");
    println!("main: log length {}, depth {}", log_len(), DEPTH.get());
    println!("main: first entry {:?}", first_entry());
    // ANCHOR_END: threads
}
