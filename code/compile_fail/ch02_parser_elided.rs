// harness: compile-fail E0499
//! The same parser with an elided return type: every word now borrows from the
//! `&mut self` borrow, so it keeps the parser exclusively borrowed.

pub struct Parser<'src> {
    input: &'src str,
    pos: usize,
}

impl<'src> Parser<'src> {
    pub fn new(input: &'src str) -> Self {
        Parser { input, pos: 0 }
    }

    // ANCHOR: elided
    // Elided: the result has the lifetime of the `&mut self` borrow.
    pub fn next_word(&mut self) -> Option<&str> {
        // ANCHOR_END: elided
        let input = self.input;
        let rest = &input[self.pos..];
        let trimmed = rest.trim_start();
        if trimmed.is_empty() {
            return None;
        }
        let start = self.pos + (rest.len() - trimmed.len());
        let end = trimmed
            .find(char::is_whitespace)
            .map_or(input.len(), |i| start + i);
        self.pos = end;
        Some(&input[start..end])
    }
}

// ANCHOR: caller
pub fn caller() {
    let text = String::from("lifetimes are regions");
    let mut parser = Parser::new(&text);
    let first = parser.next_word();
    let second = parser.next_word(); // a second `&mut parser` while `first` is live
    println!("{first:?} {second:?}");
}
// ANCHOR_END: caller
