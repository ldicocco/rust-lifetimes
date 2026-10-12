// harness: run
//! When elision cannot decide, you state the contract. There are three honest
//! choices, and each one means something different to the caller.
//!
//! Run with: `cargo run --example ch03_three_contracts`

// ANCHOR: contracts
// 1. The result may come from EITHER input: tie all three together.
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

// 2. The result comes only from `x`: say so, and leave `y` independent.
fn first_only<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// 3. The result borrows from neither: return an owned value.
fn joined(x: &str, y: &str) -> String {
    format!("{x}{y}")
}
// ANCHOR_END: contracts

// ANCHOR: caller
fn main() {
    let long_lived = String::from("long-lived");
    let kept;
    {
        let short_lived = String::from("short");
        kept = first_only(&long_lived, &short_lived); // does not borrow `short_lived`
        println!("{}", longer(&long_lived, &short_lived)); // used before the `}`
        println!("{}", joined(&long_lived, &short_lived));
    }
    println!("{kept}");
}
// ANCHOR_END: caller
