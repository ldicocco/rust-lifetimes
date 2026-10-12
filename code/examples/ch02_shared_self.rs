// harness: run
//! `&'src self` does not lock the parser: shared references are covariant, so
//! the compiler can shorten `'src` at the call. It buys nothing either.
//!
//! Run with: `cargo run --example ch02_shared_self`

struct Parser<'src> {
    input: &'src str,
    pos: usize,
}

impl<'src> Parser<'src> {
    fn new(input: &'src str) -> Self {
        Parser { input, pos: 0 }
    }
}

// ANCHOR: shared_self
impl<'src> Parser<'src> {
    fn rest(&'src self) -> &'src str {
        &self.input[self.pos..]
    }

    fn advance(&mut self, bytes: usize) {
        self.pos = (self.pos + bytes).min(self.input.len());
    }
}

fn main() {
    let text = String::from("lifetimes are regions");
    let mut parser = Parser::new(&text);
    let before = parser.rest();
    println!("{before}"); // last use of `before`
    parser.advance(10); // fine: the shared borrow has ended
    println!("{}", parser.rest());
}
// ANCHOR_END: shared_self
