// harness: run
//! Workaround 2: move the lifetime from a GAT to a helper trait, as in Sabrina
//! Jewson's "The Better Alternative to Lifetime GATs". A defaulted type
//! parameter supplies the implied bound `Self: 'this`.
//!
//! Run with: `cargo run --example ch10_helper_trait`

use std::fmt::Debug;

// ANCHOR: traits
mod sealed {
    pub trait Sealed: Sized {}
    pub struct Bounds<T>(T);
    impl<T> Sealed for Bounds<T> {}
}
use sealed::{Bounds, Sealed};

/// The item type of a lending iterator, for one lifetime `'this`. The second
/// parameter is never written by users: its default, `Bounds<&'this Self>`,
/// only exists to imply `Self: 'this`.
pub trait LendingIteratorLifetime<'this, ImplicitBounds: Sealed = Bounds<&'this Self>> {
    type Item;
}

pub trait LendingIterator: for<'this> LendingIteratorLifetime<'this> {
    fn next(&mut self) -> Option<<Self as LendingIteratorLifetime<'_>>::Item>;
}
// ANCHOR_END: traits

pub struct WindowsMut<'s, T> {
    data: &'s mut [T],
    size: usize,
    start: usize,
}

// ANCHOR: impls
impl<'this, 's, T> LendingIteratorLifetime<'this> for WindowsMut<'s, T> {
    type Item = &'this mut [T];
}

impl<'s, T> LendingIterator for WindowsMut<'s, T> {
    fn next(&mut self) -> Option<&mut [T]> {
        let start = self.start;
        let end = start.checked_add(self.size)?;
        let window = self.data.get_mut(start..end)?;
        self.start += 1;
        Some(window)
    }
}
// ANCHOR_END: impls

// ANCHOR: generic
pub fn print_all<L>(mut lender: L)
where
    L: LendingIterator,
    for<'a> <L as LendingIteratorLifetime<'a>>::Item: Debug,
{
    while let Some(item) = lender.next() {
        println!("{item:?}");
    }
}

pub fn for_each<L, F>(mut lender: L, mut f: F)
where
    L: LendingIterator,
    F: for<'a> FnMut(<L as LendingIteratorLifetime<'a>>::Item),
{
    while let Some(item) = lender.next() {
        f(item);
    }
}
// ANCHOR_END: generic

fn main() {
    // ANCHOR: main
    let mut readings = vec![3, 5, 4, 8];
    print_all(WindowsMut {
        data: &mut readings,
        size: 2,
        start: 0,
    });
    for_each(
        WindowsMut {
            data: &mut readings,
            size: 3,
            start: 0,
        },
        |triple| triple.reverse(),
    );
    println!("{readings:?}");
    // ANCHOR_END: main
}
