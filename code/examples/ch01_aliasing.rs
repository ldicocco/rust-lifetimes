// harness: run
//! Aliasing XOR mutation: many shared borrows at once, or one mutable borrow,
//! at every point of the program. Borrows that do not overlap never conflict.
//!
//! Run with: `cargo run --example ch01_aliasing`

// ANCHOR: aliasing
fn main() {
    let mut log = String::from("== report ==");

    // Any number of shared borrows may be live at the same time.
    let whole = &log;
    let title = &log[3..9];
    println!("{whole} / {title}");

    // One mutable borrow at a time: each one ends before the next begins.
    let header = &mut log;
    header.push_str("\nstatus: ");
    let body = &mut log;
    body.push_str("all good");

    println!("{log}");
}
// ANCHOR_END: aliasing
