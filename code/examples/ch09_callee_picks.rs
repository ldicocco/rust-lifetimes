// harness: run
//! A higher-ranked bound: `f` must accept a borrow of any lifetime, including
//! one that exists only inside `with_local`.
//!
//! Run with: `cargo run --example ch09_callee_picks`

// ANCHOR: with_local
fn with_local<F>(f: F) -> usize
where
    F: for<'a> Fn(&'a str) -> usize, // the same as `F: Fn(&str) -> usize`
{
    let local = String::from("created inside");
    f(&local) + f("a literal works too")
}
// ANCHOR_END: with_local

fn main() {
    println!("{}", with_local(|s| s.len()));
}
