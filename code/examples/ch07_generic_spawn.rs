// harness: run
//! The fix for E0310: the `'static` requirement of `thread::spawn` is written on
//! the generic parameter, where every caller can see it.
//!
//! Run with: `cargo run --example ch07_generic_spawn`

use std::thread;

// ANCHOR: fixed
fn drop_in_background<T: Send + 'static>(value: T) -> thread::JoinHandle<()> {
    thread::spawn(move || drop(value))
}
// ANCHOR_END: fixed

struct Table(Vec<String>);

impl Drop for Table {
    fn drop(&mut self) {
        println!("dropping a table of {} rows", self.0.len());
    }
}

fn main() {
    // ANCHOR: main
    let table = Table((0..1000).map(|i| format!("row {i}")).collect());
    let handle = drop_in_background(table); // `Table: Send + 'static`
    handle.join().unwrap();
    println!("joined");
    // ANCHOR_END: main
}
