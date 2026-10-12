// harness: run
//! The usual fix when a thread needs a string: take ownership of it. Callers
//! can pass a literal or a `String`.
//!
//! Run with: `cargo run --example ch07_logger`

use std::thread;

// ANCHOR: owned_param
fn start_logger(prefix: impl Into<String>) -> thread::JoinHandle<()> {
    let prefix: String = prefix.into(); // owned, so the closure is `'static`
    thread::spawn(move || println!("{prefix}: started"))
}
// ANCHOR_END: owned_param

fn main() {
    // ANCHOR: main
    start_logger("main").join().unwrap(); // a literal
    for id in 1..=2 {
        start_logger(format!("worker-{id}")).join().unwrap(); // a `String`
    }
    // ANCHOR_END: main
}
