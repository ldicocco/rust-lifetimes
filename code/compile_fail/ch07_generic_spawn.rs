// harness: compile-fail E0310
//! A generic function that passes its `T` to `thread::spawn` must promise
//! `T: 'static` in its own signature.

use std::thread;

// ANCHOR: generic
pub fn drop_in_background<T: Send>(value: T) -> thread::JoinHandle<()> {
    thread::spawn(move || drop(value))
}
// ANCHOR_END: generic
