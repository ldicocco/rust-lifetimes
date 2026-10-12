// harness: run
//! `thread::scope` joins every scoped thread before it returns, whether or not
//! you join the handles yourself.
//!
//! Run with: `cargo run --example ch13_scope`

use std::thread;

fn main() {
    // ANCHOR: scope
    let words = vec!["listen", "port", "8080"];
    let mut total = 0;
    thread::scope(|s| {
        let lengths = s.spawn(|| words.iter().map(|w| w.len()).sum::<usize>());
        // This handle is dropped without `join`: the scope still waits for it.
        s.spawn(|| println!("a scoped thread sees {} words", words.len()));
        total = lengths.join().unwrap();
    });
    println!("after the scope: {total} bytes, and `words` is ours again: {words:?}");
    // ANCHOR_END: scope
}
