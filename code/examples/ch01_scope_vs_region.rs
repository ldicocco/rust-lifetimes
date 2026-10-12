// harness: run
//! The scope of a value, the scope of the variable that holds a reference, and
//! the region of the borrow are three different things.
//!
//! Run with: `cargo run --example ch01_scope_vs_region`

// ANCHOR: scopes
fn main() {
    let config = String::from("port=8080"); // `config` comes into scope
    let key = &config[..4]; // the borrow is created
    println!("key: {key}"); // last use of `key`: the borrow's region ends here

    let moved = config; // fine: no borrow of `config` is live any more
    println!("moved: {moved}");
} // `key` and `moved` go out of scope here
// ANCHOR_END: scopes
