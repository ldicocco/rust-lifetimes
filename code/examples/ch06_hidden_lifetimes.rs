// harness: run
//! Job 1 of `'_`: making the lifetime parameters of types such as
//! `SplitWhitespace<'a>`, `fmt::Formatter<'a>` and `MutexGuard<'a, T>` visible.
//! The lint `elided_lifetimes_in_paths` is denied, so this program compiles only
//! because no lifetime parameter is hidden, in signatures or in the body.
//!
//! Run with: `cargo run --example ch06_hidden_lifetimes`

#![deny(elided_lifetimes_in_paths)]

use std::fmt;
use std::str::SplitWhitespace;
use std::sync::{Mutex, MutexGuard};

// ANCHOR: words
// In the return type, `'_` is the lifetime elision picks: the one of `text`.
fn words(text: &str) -> SplitWhitespace<'_> {
    text.split_whitespace()
}

// In a parameter, `'_` is a fresh lifetime parameter, as if it were elided.
fn count_long(words: SplitWhitespace<'_>, min_len: usize) -> usize {
    words.filter(|w| w.len() >= min_len).count()
}
// ANCHOR_END: words

// ANCHOR: elided_mix
// `&'_ str` and `&str` are both "elided": using one in the parameter and the
// other in the return type is consistent, and the lint stays silent.
fn trimmed(text: &'_ str) -> &str {
    text.trim()
}
// ANCHOR_END: elided_mix

// ANCHOR: formatter
struct Port(u16);

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "port {}", self.0)
    }
}
// ANCHOR_END: formatter

// ANCHOR: body
fn record(log: &Mutex<Vec<String>>, line: &str) -> usize {
    // In a body, `'_` stands for "infer this lifetime": the guard borrows `log`.
    let mut guard: MutexGuard<'_, Vec<String>> = log.lock().unwrap();
    guard.push(trimmed(line).to_string());
    guard.len()
}
// ANCHOR_END: body

fn main() {
    let line = String::from("  listen on port 8080 for incoming requests  ");
    println!("{} long words", count_long(words(trimmed(&line)), 6));
    let log = Mutex::new(Vec::new());
    record(&log, &line);
    let entries = record(&log, &Port(8080).to_string());
    println!("{entries} entries: {:?}", log.lock().unwrap());
}
