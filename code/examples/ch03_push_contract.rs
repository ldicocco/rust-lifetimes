// harness: run
//! The contract that `push` actually needs: the new item must live as long as the
//! items already in the vector, and the `&mut` borrow of the vector is separate
//! and short.
//!
//! Run with: `cargo run --example ch03_push_contract`

// ANCHOR: push
fn push<'a>(list: &mut Vec<&'a str>, item: &'a str) {
    list.push(item);
}
// ANCHOR_END: push

// ANCHOR: caller
fn main() {
    let first = String::from("first");
    let second = String::from("second");
    let mut list: Vec<&str> = Vec::new();
    push(&mut list, &first);
    push(&mut list, &second); // the first `&mut list` has already ended
    println!("{} items: {list:?}", list.len());
}
// ANCHOR_END: caller
