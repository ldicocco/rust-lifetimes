// harness: compile-fail E0499
//! `&'src mut self` in `impl<'src> Parser<'src>` borrows the parser for the
//! whole of `'src`: after the first call it can never be used again.

pub struct Parser<'src> {
    input: &'src str,
    pos: usize,
}

impl<'src> Parser<'src> {
    pub fn new(input: &'src str) -> Self {
        Parser { input, pos: 0 }
    }

    // ANCHOR: forever
    // The same name for the borrow of the parser and for the text it reads.
    pub fn next_word(&'src mut self) -> Option<&'src str> {
        // ANCHOR_END: forever
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
pub fn caller(text: &str) {
    let mut parser = Parser::new(text);
    parser.next_word(); // the result is thrown away...
    parser.next_word(); // ...and the parser is still borrowed
}
// ANCHOR_END: caller
