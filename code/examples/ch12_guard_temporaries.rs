// harness: run
//! A guard created inside an expression is a temporary, and it follows the
//! temporary scope rules of Chapter 5: it can stay alive much longer than the
//! value you were interested in.
//!
//! Run with: `cargo run --example ch12_guard_temporaries`

use std::cell::RefCell;

// ANCHOR: status
/// Reports whether the cell is borrowed right now, without panicking.
fn status(words: &RefCell<Vec<String>>) -> &'static str {
    if words.try_borrow_mut().is_ok() {
        "free"
    } else {
        "borrowed"
    }
}
// ANCHOR_END: status

// ANCHOR: tail
/// Rejected in edition 2021 (`compile_fail/ch12_tail_guard_2021.rs`).
fn entry_count() -> usize {
    let history = RefCell::new(vec![String::from("open")]);
    history.borrow().len()
}
// ANCHOR_END: tail

fn main() {
    let words = RefCell::new(vec![String::from("alpha"), String::from("beta")]);

    // ANCHOR: match
    match words.borrow().len() {
        0 => println!("match: empty"),
        n => println!("match: {n} words, cell is {}", status(&words)),
    }
    // ANCHOR_END: match

    // ANCHOR: if_let
    if let Some(first) = words.borrow().first().cloned() {
        println!("if let, then-branch: {first}, cell is {}", status(&words));
    }
    if let Some(tenth) = words.borrow().get(9).cloned() {
        println!("if let, then-branch: {tenth}");
    } else {
        println!("if let, else-branch: cell is {}", status(&words));
    }
    // ANCHOR_END: if_let

    // ANCHOR: let_first
    let count = words.borrow().len(); // the guard dies at this `;`
    println!("after let: {count} words, cell is {}", status(&words));
    // ANCHOR_END: let_first

    // ANCHOR: extended
    {
        let first = &words.borrow()[0]; // the guard is lifetime-extended
        println!("extended: {first}, cell is {}", status(&words));
    }
    // ANCHOR_END: extended

    // ANCHOR: while_let
    while let Some(word) = words.borrow_mut().pop() {
        println!("while let body: {word}, cell is {}", status(&words));
    }
    // ANCHOR_END: while_let

    println!("entry_count() = {}", entry_count());
}
