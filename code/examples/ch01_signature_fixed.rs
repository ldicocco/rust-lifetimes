// harness: run
//! Give `y` its own lifetime and the signature says what the body does: the
//! result borrows from `x` only.
//!
//! Run with: `cargo run --example ch01_signature_fixed`

// ANCHOR: first
fn first<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    println!("ignoring {y:?}");
    x
}
// ANCHOR_END: first

fn main() {
    let outer = String::from("a longer string");
    let result;
    {
        let inner = String::from("short");
        result = first(outer.as_str(), inner.as_str());
    }
    println!("{result}"); // fine: `result` does not borrow from `inner`
}
