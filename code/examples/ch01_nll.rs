// harness: run
//! Non-lexical lifetimes: a borrow is live from its creation to its last use,
//! not to the end of the enclosing block.
//!
//! Run with: `cargo run --example ch01_nll`

// ANCHOR: nll
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    println!("{first}"); // last use of `first`: the borrow ends here
    v.push(4); // so this mutable borrow does not overlap it
    println!("{v:?}");
}
// ANCHOR_END: nll
