// harness: compile-fail E0502
//! Borrowed tokens cannot be kept after their buffer is cleared for the next
//! line: the buffer is still borrowed by the tokens in `history`.

#[derive(Debug, Clone, Copy)]
pub enum Token<'src> {
    Word(&'src str),
}

pub fn tokenize(src: &str) -> impl Iterator<Item = Token<'_>> {
    src.split_whitespace().map(Token::Word)
}

// ANCHOR: keep_borrowed
pub fn caller(input: &[&str]) -> usize {
    let mut line = String::new();
    let mut history: Vec<Token<'_>> = Vec::new();
    for text in input {
        line.clear(); // needs `&mut line`...
        line.push_str(text);
        history.extend(tokenize(&line)); // ...but `history` borrows `line`
    }
    history.len()
}
// ANCHOR_END: keep_borrowed
