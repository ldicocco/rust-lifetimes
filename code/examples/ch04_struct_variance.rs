// harness: run
//! The compiler computes the variance of a struct from its fields. Each witness
//! below compiles, so it proves the variance claimed in its comment.
//!
//! Run with: `cargo run --example ch04_struct_variance`

use std::cell::Cell;

// ANCHOR: structs
// Only read through: covariant in 'src.
struct Tokens<'src> {
    text: &'src str,
    position: usize,
}

// 'a appears only as an argument of a fn pointer: contravariant in 'a.
struct Handler<'a> {
    on_line: fn(&'a str) -> usize,
}

// Written through a shared reference: invariant in 'a.
struct Slot<'a> {
    current: Cell<&'a str>,
}
// ANCHOR_END: structs

// ANCHOR: witnesses
// Covariant: Tokens<'long> can be used as Tokens<'short>.
fn shrink_tokens<'long: 'short, 'short>(t: Tokens<'long>) -> Tokens<'short> {
    t
}

// Contravariant: a handler for any short-lived line can handle 'static lines.
fn handler_for_static<'a>(h: Handler<'a>) -> Handler<'static> {
    h
}

// Invariant: the only conversion is to the very same type.
fn same_slot<'a>(s: Slot<'a>) -> Slot<'a> {
    s
}
// ANCHOR_END: witnesses

fn line_length(line: &str) -> usize {
    line.len()
}

fn main() {
    let tokens = shrink_tokens(Tokens {
        text: "let x = 1;",
        position: 4,
    });
    println!("rest: {:?}", &tokens.text[tokens.position..]);

    let handler = handler_for_static(Handler {
        on_line: line_length,
    });
    println!("length: {}", (handler.on_line)("a 'static line"));

    let slot = same_slot(Slot {
        current: Cell::new("old"),
    });
    slot.current.set("new");
    println!("slot: {}", slot.current.get());
}
