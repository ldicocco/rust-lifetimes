// harness: run
//! The classic `longest`: one lifetime name ties the result to both inputs.
//!
//! Run with: `cargo run --example ch01_longest`

// ANCHOR: longest
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}
// ANCHOR_END: longest

// ANCHOR: use_inside
fn use_inside() {
    let outer = String::from("a longer string");
    let result;
    {
        let inner = String::from("short");
        result = longest(outer.as_str(), inner.as_str());
        println!("{result}"); // fine: `inner` is still alive here
    }
}
// ANCHOR_END: use_inside

// ANCHOR: own_result
fn own_result() {
    let outer = String::from("a longer string");
    let result: String;
    {
        let inner = String::from("short");
        result = longest(outer.as_str(), inner.as_str()).to_owned(); // a copy
    }
    println!("{result}"); // fine: `result` borrows nothing
}
// ANCHOR_END: own_result

fn main() {
    use_inside();
    own_result();
}
