// harness: run
//! `&'src Collector<'src>` looks just as suspicious as `&'src mut Collector<'src>`,
//! but it is harmless: the shared reference is covariant in `Collector<'src>`, so
//! the compiler can shrink `'src` to the length of the borrow.
//!
//! Run with: `cargo run --example ch04_shared_self_borrow`

// ANCHOR: collector
struct Collector<'src> {
    words: Vec<&'src str>,
}

impl<'src> Collector<'src> {
    // The right signature: a short `&mut` borrow, and lines that live for 'src.
    fn add(&mut self, line: &'src str) {
        self.words.extend(line.split_whitespace());
    }

    // Over-constrained (`&'src self`), but harmless thanks to covariance.
    fn first(&'src self) -> Option<&'src str> {
        self.words.first().copied()
    }
}
// ANCHOR_END: collector

// ANCHOR: caller
fn main() {
    let text = String::from("alpha beta");
    let more = String::from("gamma");
    let mut collector = Collector { words: Vec::new() };
    collector.add(&text);
    println!("{:?}", collector.first()); // a shared borrow...
    collector.add(&more); // ...that has already ended: `add` may borrow mutably
    println!("{:?} of {}", collector.first(), collector.words.len());
}
// ANCHOR_END: caller
