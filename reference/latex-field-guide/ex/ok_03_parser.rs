use std::fmt;

struct Parser<'src> {
    input: &'src str,
    pos: usize,
}

impl<'src> Parser<'src> {
    fn new(input: &'src str) -> Self {
        Parser { input, pos: 0 }
    }

    /// The result borrows from the *text* ('src), not from the `&mut self`
    /// borrow, so earlier results stay usable across later calls.
    fn next_word(&mut self) -> Option<&'src str> {
        let input: &'src str = self.input;
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

// `'_` : "some lifetime, I don't need to name it"
impl fmt::Display for Parser<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parser@{} over {:?}", self.pos, self.input)
    }
}

fn main() {
    let text = String::from("lifetimes are regions");
    let mut p = Parser::new(&text);
    let a = p.next_word();
    let b = p.next_word(); // OK: `a` does not keep `p` borrowed
    println!("{a:?} {b:?} / {p}");
}
