// harness: run
//! Lifetimes are erased: they exist for the borrow checker only. At run time a
//! `&'static str` and a borrow of a local `String` are the same kind of value.
//!
//! Run with: `cargo run --example ch01_erased`

use std::any::type_name;
use std::mem::size_of;

// ANCHOR: erased
fn first_word(text: &str) -> &str {
    text.split(' ').next().unwrap_or("")
}

fn main() {
    let literal: &'static str = "regions not durations";
    let owned = String::from("borrowed from a local");
    let local: &str = &owned;

    // One function, one piece of machine code, both kinds of borrow.
    println!("{} / {}", first_word(literal), first_word(local));

    // The name of the type that code generation sees has no lifetime in it...
    let name = type_name::<&'static str>();
    println!("type_name::<&'static str>() = {name}");
    // ...and the two kinds of reference have the same size and layout.
    assert_eq!(size_of::<&'static str>(), size_of::<&str>());
}
// ANCHOR_END: erased
