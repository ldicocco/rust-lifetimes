// harness: compile-fail E0502
//! With `&'src self`, the result is limited by the shared borrow of the parser,
//! exactly as with an elided `&self`: no mutation while it is in use.

pub struct Parser<'src> {
    input: &'src str,
    pos: usize,
}

impl<'src> Parser<'src> {
    pub fn new(input: &'src str) -> Self {
        Parser { input, pos: 0 }
    }

    pub fn rest(&'src self) -> &'src str {
        &self.input[self.pos..]
    }

    pub fn advance(&mut self, bytes: usize) {
        self.pos = (self.pos + bytes).min(self.input.len());
    }
}

// ANCHOR: keep
pub fn caller(text: &str) {
    let mut parser = Parser::new(text);
    let before = parser.rest();
    parser.advance(10);
    println!("{before}"); // `before` keeps the shared borrow of `parser` live
}
// ANCHOR_END: keep
