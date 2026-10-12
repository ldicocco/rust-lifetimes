// harness: compile-fail E0597
//! A value of type `Parser<'src>` cannot be used outside the region `'src`:
//! the parser carries the borrow of its text.

pub struct Parser<'src> {
    input: &'src str,
    pos: usize,
}

impl<'src> Parser<'src> {
    pub fn new(input: &'src str) -> Self {
        Parser { input, pos: 0 }
    }
}

// ANCHOR: outlives
pub fn caller() {
    let parser;
    {
        let text = String::from("lifetimes are regions");
        parser = Parser::new(&text);
    } // `text` is dropped here
    println!("{} at {}", parser.input, parser.pos);
}
// ANCHOR_END: outlives
