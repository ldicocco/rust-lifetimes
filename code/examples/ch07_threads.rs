// harness: run
//! Three ways to give data to a thread: move ownership in, share ownership with
//! `Arc`, or borrow it from a `thread::scope`.
//!
//! Run with: `cargo run --example ch07_threads`

use std::sync::Arc;
use std::thread;

fn main() {
    let words = vec![
        String::from("alpha"),
        String::from("beta"),
        String::from("gamma"),
    ];

    // ANCHOR: own
    // 1. Own: the closure owns a clone, so its type is `'static`.
    let owned = words.clone();
    let handle = thread::spawn(move || owned.join(" "));
    println!("moved:  {}", handle.join().unwrap());
    // ANCHOR_END: own

    // ANCHOR: share
    // 2. Share: `Arc<Vec<String>>` owns its data, so it is `'static` too.
    let shared = Arc::new(words.clone());
    let handles: Vec<_> = (0..shared.len())
        .map(|i| {
            let shared = Arc::clone(&shared);
            thread::spawn(move || shared[i].len())
        })
        .collect();
    let lengths: Vec<usize> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("shared: {lengths:?}");
    // ANCHOR_END: share

    // ANCHOR: scope
    // 3. Borrow: every thread spawned in the scope is joined before `scope`
    //    returns, so the closures may borrow `words`.
    let (total, longest) = thread::scope(|s| {
        let total = s.spawn(|| words.iter().map(|w| w.len()).sum::<usize>());
        let longest = s.spawn(|| words.iter().max_by_key(|w| w.len()).unwrap());
        (total.join().unwrap(), longest.join().unwrap())
    });
    println!("scoped: total {total}, longest {longest}"); // `longest: &String`
    println!("still mine: {words:?}");
    // ANCHOR_END: scope
}
