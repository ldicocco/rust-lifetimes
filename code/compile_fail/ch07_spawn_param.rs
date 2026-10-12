// harness: compile-fail E0521
//! A `move` closure that captures a borrowed parameter still holds a borrow, so
//! it cannot be handed to `thread::spawn`, even if the thread is joined at once.

use std::thread;

// ANCHOR: spawn_param
pub fn count_in_background(words: &[String]) -> usize {
    let handle = thread::spawn(move || words.len()); // moves the *reference* in
    handle.join().unwrap()
}
// ANCHOR_END: spawn_param
