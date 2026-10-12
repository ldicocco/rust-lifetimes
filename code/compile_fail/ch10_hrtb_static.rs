// harness: compile-fail E0597 "current limitation of the type system"
//! A higher-ranked bound on a GAT, `for<'a> L::Item<'a>: Debug`, forces the
//! lender to be `'static`.

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

// ANCHOR: print_all
pub fn print_all<L>(mut lender: L)
where
    L: LendingIterator,
    for<'a> L::Item<'a>: Debug,
{
    while let Some(item) = lender.next() {
        println!("{item:?}");
    }
}
// ANCHOR_END: print_all

// ANCHOR: caller
pub fn caller() {
    let config = String::from("port 8080\nlog-level debug");
    let lines = NormalizedLines {
        lines: config.lines(),
        buffer: String::new(),
    };
    print_all(lines);
}
// ANCHOR_END: caller
