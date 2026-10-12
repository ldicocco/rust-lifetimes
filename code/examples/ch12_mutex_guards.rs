// harness: run
//! `MutexGuard` follows the same rules as `Ref`: it is a borrow of the mutex, and
//! the lock is held until the guard is dropped. Holding it longer than needed
//! blocks other threads; locking again on the same thread never returns.
//!
//! Run with: `cargo run --example ch12_mutex_guards`

use std::sync::{Mutex, TryLockError};
use std::thread;

// ANCHOR: status
/// Reports whether the mutex is locked right now, without blocking.
fn status<T>(mutex: &Mutex<T>) -> &'static str {
    match mutex.try_lock() {
        Ok(_) => "free",
        Err(TryLockError::WouldBlock) => "locked",
        Err(TryLockError::Poisoned(_)) => "poisoned",
    }
}
// ANCHOR_END: status

fn main() {
    // ANCHOR: scoped
    let lengths = Mutex::new(Vec::new());
    thread::scope(|s| {
        for word in ["alpha", "beta", "gamma"] {
            let lengths = &lengths; // a plain shared borrow: no `Arc` needed
            s.spawn(move || {
                let len = word.len(); // work without holding the lock
                lengths.lock().unwrap().push((word, len)); // lock, push, unlock
            });
        }
    });
    let mut lengths = lengths.into_inner().unwrap();
    lengths.sort();
    println!("{lengths:?}");
    // ANCHOR_END: scoped

    // ANCHOR: block
    let queue = Mutex::new(vec!["parse", "check"]);
    {
        let mut jobs = queue.lock().unwrap();
        jobs.push("emit");
        println!("inside the block: {}", status(&queue));
    } // `jobs` is dropped: unlocked
    println!("after the block: {}", status(&queue));
    // ANCHOR_END: block

    // ANCHOR: while_let
    while let Some(job) = queue.lock().unwrap().pop() {
        // Still locked here: calling `queue.lock()` would never return.
        println!("while let, job {job}: {}", status(&queue));
    }
    // ANCHOR_END: while_let

    queue.lock().unwrap().extend(["parse", "check"]);

    // ANCHOR: loop_fix
    loop {
        let next = queue.lock().unwrap().pop(); // the guard dies at this `;`
        let Some(job) = next else { break };
        println!("loop, job {job}: {}", status(&queue));
    }
    // ANCHOR_END: loop_fix
}
