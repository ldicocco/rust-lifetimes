// harness: compile-fail E0502
//! In every edition, the temporaries of a `match` scrutinee live until the end of
//! the `match`: the arms run while they are alive.

pub struct Lines<'a> {
    lines: &'a Vec<String>,
}

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

// ANCHOR: match
pub fn show_first(lines: &mut Vec<String>) {
    match Lines::new(lines).first() {
        Some(first) => println!("first line: {first}"),
        None => lines.push(String::from("(empty)")),
    }
}
// ANCHOR_END: match
