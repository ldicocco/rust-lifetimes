// harness: run
//! Zero-copy tokenizing: `Token<'src>` borrows its text from the source, so
//! producing a token allocates nothing. `OwnedToken` is the version for storage.
//!
//! Run with: `cargo run --example ch14_tokenizer`

// ANCHOR: token
/// A token whose text is a slice of the source it was read from.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Token<'src> {
    Ident(&'src str),
    Number(&'src str),
    Str(&'src str), // the contents, without the quotes
    Punct(char),
}
// ANCHOR_END: token

// ANCHOR: tokenizer
struct Tokenizer<'src> {
    rest: &'src str,
}

impl<'src> Tokenizer<'src> {
    fn new(src: &'src str) -> Self {
        Tokenizer { rest: src }
    }
}

impl<'src> Iterator for Tokenizer<'src> {
    type Item = Token<'src>; // items borrow from the source, not from the tokenizer

    fn next(&mut self) -> Option<Token<'src>> {
        let rest: &'src str = self.rest.trim_start(); // copy the `&'src str` out
        let first = rest.chars().next()?;
        let word_end =
            |pred: fn(char) -> bool| rest.find(|c| !pred(c)).unwrap_or(rest.len());
        let (token, len) = if first.is_alphabetic() || first == '_' {
            let len = word_end(|c| c.is_alphanumeric() || c == '_');
            (Token::Ident(&rest[..len]), len)
        } else if first.is_ascii_digit() {
            let len = word_end(|c| c.is_ascii_digit());
            (Token::Number(&rest[..len]), len)
        } else if first == '"' {
            let close = 1 + rest[1..].find('"')?; // an unterminated string ends the input
            (Token::Str(&rest[1..close]), close + 1)
        } else {
            (Token::Punct(first), first.len_utf8())
        };
        self.rest = &rest[len..];
        Some(token)
    }
}
// ANCHOR_END: tokenizer

// ANCHOR: owned
/// The same token, owning its text: it can outlive the source.
#[derive(Debug, Clone, PartialEq)]
enum OwnedToken {
    Ident(String),
    Number(String),
    Str(String),
    Punct(char),
}

impl Token<'_> {
    /// Copies the borrowed text, so the result no longer depends on the source.
    fn into_owned(self) -> OwnedToken {
        match self {
            Token::Ident(s) => OwnedToken::Ident(s.to_owned()),
            Token::Number(s) => OwnedToken::Number(s.to_owned()),
            Token::Str(s) => OwnedToken::Str(s.to_owned()),
            Token::Punct(c) => OwnedToken::Punct(c),
        }
    }
}
// ANCHOR_END: owned

fn main() {
    // ANCHOR: main_borrowed
    let source = String::from(r#"let total = price * 3 + "tax";"#);
    let tokens: Vec<Token<'_>> = Tokenizer::new(&source).collect();
    println!("{tokens:?}");

    // Zero-copy: the text of every token points into `source`.
    let range = source.as_bytes().as_ptr_range();
    for token in &tokens {
        if let Token::Ident(text) | Token::Number(text) | Token::Str(text) = token {
            assert!(range.contains(&text.as_ptr()));
        }
    }
    // ANCHOR_END: main_borrowed

    // ANCHOR: main_owned
    // Reading line by line into one reused buffer, as `BufRead::read_line` does.
    let input = ["x = 1", "y = x + 2"];
    let mut line = String::new();
    let mut history: Vec<OwnedToken> = Vec::new();
    for text in input {
        line.clear();
        line.push_str(text);
        history.extend(Tokenizer::new(&line).map(Token::into_owned));
    }
    let last = history.last();
    println!("kept {} tokens, the last is {last:?}", history.len());
    // ANCHOR_END: main_owned
}
