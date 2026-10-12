// harness: run
//! `Cow<'a, str>`: borrow when you can, own when you must. As a return type it
//! avoids allocating for inputs that need no change; as `Cow<'static, str>` in a
//! field it holds literals or run-time strings without a lifetime parameter.
//!
//! Run with: `cargo run --example ch14_cow`

use std::borrow::Cow;
use std::thread;

// ANCHOR: escape
/// Escapes the three characters that are special in markup. Most text contains
/// none of them, and then the result borrows the input.
fn escape(input: &str) -> Cow<'_, str> {
    if !input.contains(['<', '>', '&']) {
        return Cow::Borrowed(input);
    }
    let mut escaped = String::with_capacity(input.len() + 8);
    for c in input.chars() {
        match c {
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '&' => escaped.push_str("&amp;"),
            _ => escaped.push(c),
        }
    }
    Cow::Owned(escaped)
}
// ANCHOR_END: escape

fn describe(text: &Cow<'_, str>) -> &'static str {
    match text {
        Cow::Borrowed(_) => "borrowed",
        Cow::Owned(_) => "owned",
    }
}

// ANCHOR: column
/// A column name is usually a literal, sometimes built at run time.
#[derive(Debug)]
struct Column {
    name: Cow<'static, str>,
}

impl Column {
    fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Column { name: name.into() }
    }
}
// ANCHOR_END: column

fn main() {
    // ANCHOR: main_escape
    for input in ["plain text", "a < b && c"] {
        let escaped = escape(input);
        println!("{:<9} {escaped}", describe(&escaped));
    }
    let kept: String = escape("x > 0").into_owned(); // detach from the input
    println!("kept      {kept}");
    // ANCHOR_END: main_escape

    // ANCHOR: main_column
    let columns = vec![Column::new("id"), Column::new(format!("score_{}", 2026))];
    for column in &columns {
        println!("{:<9} {}", describe(&column.name), column.name);
    }
    // `Column: 'static`, so the columns can move to another thread.
    let handle = thread::spawn(move || columns.len());
    println!("{} columns sent to a thread", handle.join().unwrap());
    // ANCHOR_END: main_column
}
