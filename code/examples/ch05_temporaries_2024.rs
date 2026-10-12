// harness: run
//! The two programs that edition 2021 rejects (`compile_fail/ch05_tail_expr_2021.rs`
//! and `compile_fail/ch05_if_let_2021.rs`) compile unchanged in edition 2024. The
//! `match` version needs a `let` in every edition.
//!
//! Run with: `cargo run --example ch05_temporaries_2024`

struct Reader<'a> {
    text: &'a str,
}

impl<'a> Reader<'a> {
    fn new(text: &'a str) -> Self {
        Reader { text }
    }

    fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }
}

impl Drop for Reader<'_> {
    fn drop(&mut self) {}
}

fn count_words() -> usize {
    let text = String::from("one two three");
    Reader::new(&text).word_count()
}

struct Lines<'a> {
    lines: &'a Vec<String>,
}

impl<'a> Lines<'a> {
    fn new(lines: &'a Vec<String>) -> Self {
        Lines { lines }
    }

    fn first(&self) -> Option<&'a str> {
        self.lines.first().map(|line| line.as_str())
    }
}

impl Drop for Lines<'_> {
    fn drop(&mut self) {}
}

fn show_first(lines: &mut Vec<String>) {
    if let Some(first) = Lines::new(lines).first() {
        println!("first line: {first}");
    } else {
        lines.push(String::from("(empty)"));
    }
}

// ANCHOR: match_fix
fn show_first_match(lines: &mut Vec<String>) {
    let first = Lines::new(lines).first(); // the temporary dies at this `;`
    match first {
        Some(first) => println!("first line: {first}"),
        None => lines.push(String::from("(empty)")),
    }
}
// ANCHOR_END: match_fix

fn main() {
    println!("words: {}", count_words());

    let mut lines = Vec::new();
    show_first(&mut lines); // empty: the `else` branch pushes a line
    show_first(&mut lines); // prints it

    let mut other = Vec::new();
    show_first_match(&mut other);
    show_first_match(&mut other);
    assert_eq!(lines, other);
}
