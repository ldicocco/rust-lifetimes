// harness: run
//! A GAT whose lifetime comes from an input other than `self` needs no
//! `where Self: 'a`: no method implies that bound.
//!
//! Run with: `cargo run --example ch10_parse_gat`

// ANCHOR: trait
pub trait Parse {
    /// The parsed form of an input that lives for `'a`.
    type Output<'a>;

    fn parse<'a>(input: &'a str) -> Self::Output<'a>;
}
// ANCHOR_END: trait

// ANCHOR: impls
/// Borrows from the input: the words themselves.
pub struct Words;

impl Parse for Words {
    type Output<'a> = Vec<&'a str>;

    fn parse<'a>(input: &'a str) -> Vec<&'a str> {
        input.split_whitespace().collect()
    }
}

/// Borrows nothing: just a count.
pub struct WordCount;

impl Parse for WordCount {
    type Output<'a> = usize;

    fn parse(input: &str) -> usize {
        input.split_whitespace().count()
    }
}
// ANCHOR_END: impls

fn main() {
    let line = String::from("listen port 8080");
    println!("{:?}", Words::parse(&line));
    println!("{}", WordCount::parse(&line));
}
