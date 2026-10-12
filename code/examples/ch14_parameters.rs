// harness: run
//! Own or borrow: three ways to accept a string, and two ways to return one.
//!
//! Run with: `cargo run --example ch14_parameters`

// ANCHOR: borrow
/// Reads its argument during the call only: borrow it.
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}
// ANCHOR_END: borrow

// ANCHOR: as_ref
/// Also reads only, but accepts anything that can lend a `&str`, by value or by
/// reference.
fn count_lines(text: impl AsRef<str>) -> usize {
    text.as_ref().lines().count()
}
// ANCHOR_END: as_ref

// ANCHOR: into
/// Keeps its argument, so it takes ownership. With `impl Into<String>` the
/// caller passes a literal (one allocation here) or a `String` (moved, no copy).
struct Label {
    name: String,
}

impl Label {
    fn new(name: impl Into<String>) -> Self {
        Label { name: name.into() }
    }
}
// ANCHOR_END: into

// ANCHOR: returns
/// Returns a view into its input: the result borrows from `text` (elision).
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

/// Returns new data: the result is owned, and the signature has no lifetime.
fn shout(text: &str) -> String {
    text.to_uppercase()
}
// ANCHOR_END: returns

fn main() {
    // ANCHOR: main
    let owned = String::from("lifetimes are regions\nnot durations");
    println!("words: {}", count_words("a literal")); // `&'static str`
    println!("words: {}", count_words(&owned)); // `&String` coerces to `&str`
    println!("words: {}", count_words(&owned[..9])); // a slice of a `String`
    println!("lines: {}", count_lines(&owned)); // a reference...
    println!("lines: {}", count_lines(String::from("one line"))); // ...or a value

    let fixed = Label::new("fixed"); // allocates a `String`
    let built = Label::new(format!("label-{}", 2)); // moves the `String` in
    println!("labels: {}, {}", fixed.name, built.name);

    let first = first_word(&owned); // borrows from `owned`
    println!("first: {first}, shouted: {}", shout(first));
    // ANCHOR_END: main
}
