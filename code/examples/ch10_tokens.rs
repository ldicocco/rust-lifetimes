// harness: run
//! When items borrow from the data rather than from the iterator, a lifetime
//! parameter on the trait is simpler than a GAT, and items can be kept.
//!
//! Run with: `cargo run --example ch10_tokens`

// ANCHOR: trait
/// A source of tokens that borrow from a text of lifetime `'src`.
pub trait Tokens<'src> {
    fn next_token(&mut self) -> Option<&'src str>;
}
// ANCHOR_END: trait

pub struct Words<'src> {
    rest: &'src str,
}

impl<'src> Tokens<'src> for Words<'src> {
    fn next_token(&mut self) -> Option<&'src str> {
        let trimmed = self.rest.trim_start();
        if trimmed.is_empty() {
            return None;
        }
        let end = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());
        let (word, rest) = trimmed.split_at(end);
        self.rest = rest;
        Some(word)
    }
}

// ANCHOR: collect_all
/// Keeps every token: possible because a token does not borrow `tokens`.
pub fn collect_all<'src, T: Tokens<'src>>(mut tokens: T) -> Vec<&'src str> {
    let mut all = Vec::new();
    while let Some(token) = tokens.next_token() {
        all.push(token);
    }
    all
}
// ANCHOR_END: collect_all

fn main() {
    let text = String::from("listen  port 8080");
    let tokens = collect_all(Words { rest: &text });
    println!("{tokens:?}");
}
