// harness: run
//! Callbacks: a function that calls a callback before returning, a signature
//! that lets the callback keep what it receives, and a dispatcher that stores
//! callbacks which borrow local variables.
//!
//! Run with: `cargo run --example ch14_callbacks`

// ANCHOR: for_each
/// Calls `f` on every non-blank line. The callback must accept a borrow of any
/// lifetime (the bound is higher-ranked, Chapter 9), so it cannot keep it.
fn for_each_line(text: &str, mut f: impl FnMut(&str)) {
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        f(line);
    }
}
// ANCHOR_END: for_each

// ANCHOR: for_each_kept
/// The same, but the signature promises that every line borrows from `text`,
/// so a callback may keep the lines for as long as `text` lives.
fn for_each_line_kept<'text>(text: &'text str, mut f: impl FnMut(&'text str)) {
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        f(line);
    }
}
// ANCHOR_END: for_each_kept

// ANCHOR: dispatcher
/// Stores callbacks that may borrow anything that outlives `'a`.
struct Dispatcher<'a> {
    handlers: Vec<Box<dyn FnMut(&str) + 'a>>,
}

impl<'a> Dispatcher<'a> {
    fn new() -> Self {
        Dispatcher {
            handlers: Vec::new(),
        }
    }

    fn on_line(&mut self, handler: impl FnMut(&str) + 'a) {
        self.handlers.push(Box::new(handler));
    }

    fn dispatch(&mut self, line: &str) {
        for handler in &mut self.handlers {
            handler(line);
        }
    }
}
// ANCHOR_END: dispatcher

fn main() {
    let text = String::from("INFO start\n\nWARN disk almost full\nINFO stop\n");

    // ANCHOR: main_call
    let mut total = 0;
    for_each_line(&text, |line| total += line.len()); // borrows `total` mutably
    println!("total length: {total}");

    let mut kept: Vec<&str> = Vec::new();
    for_each_line_kept(&text, |line| kept.push(line)); // keeps the lines
    println!("kept: {kept:?}");
    // ANCHOR_END: main_call

    // ANCHOR: main_dispatch
    let mut count = 0;
    let mut warnings = Vec::new();
    {
        let mut dispatcher = Dispatcher::new();
        dispatcher.on_line(|_| count += 1);
        dispatcher.on_line(|line| {
            if line.starts_with("WARN") {
                warnings.push(line.to_owned()); // the line is borrowed: copy it
            }
        });
        for_each_line(&text, |line| dispatcher.dispatch(line));
    } // the dispatcher is dropped: `count` and `warnings` are free again
    println!("{count} lines, warnings: {warnings:?}");
    // ANCHOR_END: main_dispatch
}
