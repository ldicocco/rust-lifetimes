// harness: run
//! The same `print_all` is fine when the lender is `'static`: here it reads a
//! string literal, so it holds no borrow that can expire.
//!
//! Run with: `cargo run --example ch10_print_all_static`

use std::fmt::Debug;

pub trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

pub struct NormalizedLines<'src> {
    lines: std::str::Lines<'src>,
    buffer: String,
}

impl<'src> LendingIterator for NormalizedLines<'src> {
    type Item<'a>
        = &'a str
    where
        Self: 'a;

    fn next(&mut self) -> Option<&str> {
        let line = self.lines.next()?;
        self.buffer.clear();
        self.buffer.push_str(line.trim());
        Some(&self.buffer)
    }
}

pub fn print_all<L>(mut lender: L)
where
    L: LendingIterator,
    for<'a> L::Item<'a>: Debug,
{
    while let Some(item) = lender.next() {
        println!("{item:?}");
    }
}

fn main() {
    // ANCHOR: main
    // `NormalizedLines<'static>`: the text is a literal.
    let lines = NormalizedLines {
        lines: "port 8080\n  log-level debug".lines(),
        buffer: String::new(),
    };
    print_all(lines);
    // ANCHOR_END: main
}
