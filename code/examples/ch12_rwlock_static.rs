// harness: run
//! `RwLock` guards, and the guard of a `static` mutex, whose lifetime is
//! `'static` because the mutex lives forever.
//!
//! Run with: `cargo run --example ch12_rwlock_static`

use std::sync::{Mutex, MutexGuard, RwLock};

// ANCHOR: rwlock
fn rwlock_demo() {
    let config = RwLock::new(String::from("verbose=false"));
    {
        let first = config.read().unwrap();
        let second = config.read().unwrap(); // many readers at once
        let can_write = config.try_write().is_ok(); // a writer must wait
        println!("readers: {first} and {second}; can write: {can_write}");
    } // both read guards dropped
    config.write().unwrap().replace_range(8.., "true");
    println!("after writing: {}", config.read().unwrap());
}
// ANCHOR_END: rwlock

// ANCHOR: static
static LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());

fn log(line: &str) {
    LOG.lock().unwrap().push(line.to_owned());
}

/// The mutex is a `static`, so its guard can be `MutexGuard<'static, _>` and
/// can be returned from a function without any lifetime parameter.
fn log_entries() -> MutexGuard<'static, Vec<String>> {
    LOG.lock().unwrap()
}
// ANCHOR_END: static

fn main() {
    rwlock_demo();
    log("started");
    log("loaded config");
    let entries = log_entries();
    println!("log: {:?}", *entries);
}
