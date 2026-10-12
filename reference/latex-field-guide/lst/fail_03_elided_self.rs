struct Parser<'src> {
    input: &'src str,
    pos: usize,
}

impl<'src> Parser<'src> {
    // fn new(..) unchanged

    // Elision rule 3: the output lifetime is that of `&mut self`.
    fn next_word(&mut self) -> Option<&str> {
        // body unchanged: returns Some(&self.input[start..end])
    }
}

fn main() {
    let text = String::from("lifetimes are regions");
    let mut p = Parser::new(&text);
    let a = p.next_word();
    let b = p.next_word(); // second &mut borrow while `a` is live
    println!("{a:?} {b:?}");
}
