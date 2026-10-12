// harness: compile-fail E0597
//! Writing `&'static str` in a parameter silences the error inside the function
//! and moves it to every caller that does not have a literal.

use std::thread;

// ANCHOR: static_param
fn start_logger(prefix: &'static str) -> thread::JoinHandle<()> {
    thread::spawn(move || println!("{prefix}: started"))
}

pub fn caller(id: u32) {
    let prefix = format!("worker-{id}");
    start_logger(&prefix).join().unwrap();
}
// ANCHOR_END: static_param
