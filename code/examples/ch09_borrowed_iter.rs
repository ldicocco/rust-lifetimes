// harness: run
//! `for<'a> &'a C: IntoIterator`: the function owns `c` and borrows it inside
//! its body, at lifetimes no caller can name.
//!
//! Run with: `cargo run --example ch09_borrowed_iter`

use std::collections::BTreeSet;

// ANCHOR: stats
fn stats<C>(c: C) -> (i32, usize)
where
    for<'a> &'a C: IntoIterator<Item = &'a i32>,
{
    let sum = (&c).into_iter().sum();
    let count = (&c).into_iter().count();
    (sum, count)
}
// ANCHOR_END: stats

// ANCHOR: by_ref
// When the caller lends the collection, a named lifetime is enough.
fn sum_all<'x, C>(c: &'x C) -> i32
where
    &'x C: IntoIterator<Item = &'x i32>,
{
    c.into_iter().sum()
}
// ANCHOR_END: by_ref

fn main() {
    // ANCHOR: calls
    println!("{:?}", stats(vec![1, 2, 3]));
    println!("{:?}", stats([10, 20]));
    println!("{:?}", stats(BTreeSet::from([5, 6])));
    println!("{}", sum_all(&vec![1, 2, 3]));
    // ANCHOR_END: calls
}
