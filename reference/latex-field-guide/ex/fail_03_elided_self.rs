struct Parser<'src> {
    input: &'src str,
    pos: usize,
}

impl<'src> Parser<'src> {
    fn new(input: &'src str) -> Self {
        Parser { input, pos: 0 }
    }

    // Elision rule 3: the output lifetime is that of `&mut self`.
    fn next_word(&mut self) -> Option<&str> {
        let rest = &self.input[self.pos..];
        let trimmed = rest.trim_start();
        if trimmed.is_empty() {
            return None;
        }
        let start = self.pos + (rest.len() - trimmed.len());
        let end = trimmed
            .find(char::is_whitespace)
            .map_or(self.input.len(), |i| start + i);
        self.pos = end;
        Some(&self.input[start..end])
    }
}

fn main() {
    let text = String::from("lifetimes are regions");
    let mut p = Parser::new(&text);
    let a = p.next_word();
    let b = p.next_word(); // second &mut borrow while `a` is live
    println!("{a:?} {b:?}");
}
