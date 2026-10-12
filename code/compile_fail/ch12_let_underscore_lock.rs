// harness: compile-fail "non-binding let on a synchronization lock"
//! `let _ = ...` binds nothing, so the guard is dropped at the end of the
//! statement and the lock is released at once. For locks, rustc has a
//! deny-by-default lint for this mistake.

use std::sync::Mutex;

// ANCHOR: underscore
pub fn append(queue: &Mutex<Vec<String>>, job: &str) {
    let _ = queue.lock().unwrap(); // meant to hold the lock
    println!("appending {job}");
}
// ANCHOR_END: underscore
