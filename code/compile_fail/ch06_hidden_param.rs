// harness: compile-fail "hidden lifetime parameters in types are deprecated"
//! `mismatched_lifetime_syntaxes` only compares inputs with outputs, so a hidden
//! lifetime in a parameter passes it. The allow-by-default lint
//! `elided_lifetimes_in_paths` (part of `rust_2018_idioms`) catches every hidden
//! lifetime parameter; it is denied here and reports two: one in a parameter,
//! one in a `let` inside a function body.
#![deny(elided_lifetimes_in_paths)]

use std::fmt;
use std::sync::{Mutex, MutexGuard};

pub struct Port(pub u16);

// ANCHOR: formatter
impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "port {}", self.0)
    }
}
// ANCHOR_END: formatter

// ANCHOR: body
pub fn record(log: &Mutex<Vec<String>>, line: &str) -> usize {
    let mut guard: MutexGuard<Vec<String>> = log.lock().unwrap();
    guard.push(line.to_string());
    guard.len()
}
// ANCHOR_END: body
