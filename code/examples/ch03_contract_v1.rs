// harness: run
//! Version 1 of a library function: the body returns `x`, and the signature says
//! so. The caller relies on that contract.
//!
//! Run with: `cargo run --example ch03_contract_v1`

// ANCHOR: v1
fn pick<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}
// ANCHOR_END: v1

// ANCHOR: caller
fn main() {
    let long_lived = String::from("long-lived");
    let result;
    {
        let short_lived = String::from("short");
        result = pick(&long_lived, &short_lived); // relies on: not borrowed from `y`
    }
    println!("{result}");
}
// ANCHOR_END: caller
