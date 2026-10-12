// harness: compile-fail E0502 edition=2021
//! Edition 2021: the temporaries of an `if let` scrutinee live until the end of
//! the whole `if let`, `else` branch included. Edition 2024 drops them before the
//! `else` branch and accepts this code (see `code/examples/ch05_temporaries_2024.rs`).

pub struct Lines<'a> {
    lines: &'a Vec<String>,
}

// ANCHOR: lines
impl<'a> Lines<'a> {
    pub fn new(lines: &'a Vec<String>) -> Self {
        Lines { lines }
    }

    pub fn first(&self) -> Option<&'a str> {
        self.lines.first().map(|line| line.as_str())
    }
}

impl Drop for Lines<'_> {
    fn drop(&mut self) {}
}
// ANCHOR_END: lines

// ANCHOR: if_let
pub fn show_first(lines: &mut Vec<String>) {
    if let Some(first) = Lines::new(lines).first() {
        println!("first line: {first}");
    } else {
        lines.push(String::from("(empty)"));
    }
}
// ANCHOR_END: if_let
