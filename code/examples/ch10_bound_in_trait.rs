// harness: run
//! Workaround 1: put the bound on the GAT itself. Every implementation must
//! provide items that are `Debug`, and generic code needs no `for<'a>`.
//!
//! Run with: `cargo run --example ch10_bound_in_trait`

use std::fmt::Debug;

// ANCHOR: trait
pub trait LendingIterator {
    type Item<'a>: Debug
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}
// ANCHOR_END: trait

pub struct WindowsMut<'s, T> {
    data: &'s mut [T],
    size: usize,
    start: usize,
}

// ANCHOR: impl
impl<'s, T: Debug> LendingIterator for WindowsMut<'s, T> {
    type Item<'a>
        = &'a mut [T]
    where
        Self: 'a;
    // ANCHOR_END: impl

    fn next(&mut self) -> Option<&mut [T]> {
        let start = self.start;
        let end = start.checked_add(self.size)?;
        let window = self.data.get_mut(start..end)?;
        self.start += 1;
        Some(window)
    }
}

// ANCHOR: print_all
pub fn print_all<L: LendingIterator>(mut lender: L) {
    while let Some(item) = lender.next() {
        println!("{item:?}");
    }
}
// ANCHOR_END: print_all

fn main() {
    // ANCHOR: main
    let mut readings = vec![3, 5, 4, 8];
    print_all(WindowsMut {
        data: &mut readings,
        size: 2,
        start: 0,
    });
    readings.push(10); // the borrow ended when `print_all` returned
    println!("{readings:?}");
    // ANCHOR_END: main
}
