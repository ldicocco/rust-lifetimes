// harness: run
//! Temporary lifetime extension: a temporary whose borrow appears in an
//! "extending" position of a `let` initializer lives until the end of the block,
//! like a named variable.
//!
//! Run with: `cargo run --example ch05_lifetime_extension`

fn load(name: &str) -> String {
    format!("<{name}>")
}

struct Entry<'a> {
    key: &'a String,
}

fn main() {
    let quiet = false;
    // ANCHOR: extended
    let direct = &load("direct"); // operand of `&`
    let entry = Entry { key: &load("lit") }; // field of a struct literal
    let pair = (&load("tuple"), 1); // element of a tuple
    let list = [&load("array")]; // element of an array
    let block = { &load("block") }; // tail of a block
    let branch = if quiet { &load("then") } else { &load("else") }; // branches
    let arm = match quiet {
        true => &load("yes"),
        false => &load("no"), // arms of a `match`
    };
    let wrapped = Some(&load("variant")); // tuple-variant constructor
    // ANCHOR_END: extended

    println!("{direct} {} {} {}", entry.key, pair.0, list[0]);
    println!("{block} {branch} {arm} {wrapped:?}");
}
