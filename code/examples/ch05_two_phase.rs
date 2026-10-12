// harness: run
//! Two-phase borrows: the `&mut` borrow of a method receiver (or the implicit
//! reborrow of a `&mut` argument, or the `&mut` of an overloaded `+=`) is only
//! *reserved* while the other operands are evaluated, so they may read the
//! borrowed value.
//!
//! Run with: `cargo run --example ch05_two_phase`

use std::collections::HashMap;

// ANCHOR: reborrowed
fn record(lengths: &mut Vec<usize>, value: usize) {
    lengths.push(value);
}

fn record_count(lengths: &mut Vec<usize>) {
    record(lengths, lengths.len()); // the implicit reborrow is two-phase too
}
// ANCHOR_END: reborrowed

// ANCHOR: compound
#[derive(Debug)]
struct Total(usize);

impl std::ops::AddAssign<usize> for Total {
    fn add_assign(&mut self, amount: usize) {
        self.0 += amount;
    }
}
// ANCHOR_END: compound

fn main() {
    // ANCHOR: accepted
    let mut lengths = vec![3, 1];
    lengths.push(lengths.len()); // `&mut lengths` reserved, `lengths.len()` reads

    let mut ids: HashMap<String, usize> = HashMap::new();
    ids.insert(String::from("alpha"), ids.len());
    ids.insert(String::from("beta"), ids.len());

    let mut text = String::from("ab");
    text.push_str(&text.to_uppercase()); // the shared borrow ends before the call

    let first = &lengths[0];
    lengths.push(*first); // `*first` is copied out before the push is activated
    // ANCHOR_END: accepted

    // ANCHOR: compound_use
    let mut total = Total(5);
    total += total.0; // `AddAssign::add_assign(&mut total, total.0)`, two-phase
    // ANCHOR_END: compound_use

    record_count(&mut lengths);
    let mut ids: Vec<_> = ids.into_iter().collect();
    ids.sort();
    println!("lengths = {lengths:?}");
    println!("ids = {ids:?}");
    println!("text = {text}");
    println!("total = {total:?}");
}
