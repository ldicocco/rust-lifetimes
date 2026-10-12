// harness: run
//! Two lifetime parameters for a struct that holds `&mut` to a container of
//! borrows: one for the borrow of the container, one for the borrowed items.
//!
//! Run with: `cargo run --example ch14_collector`

// ANCHOR: collector
/// `'out`: how long we may write into the vector.
/// `'text`: how long the words we push stay valid.
struct Collector<'out, 'text> {
    out: &'out mut Vec<&'text str>,
}

impl<'text> Collector<'_, 'text> {
    fn push(&mut self, word: &'text str) {
        self.out.push(word);
    }
}
// ANCHOR_END: collector

fn main() {
    // ANCHOR: main
    let text = String::from("to be or not to be");
    let mut words: Vec<&str> = Vec::new();
    {
        let mut collector = Collector { out: &mut words };
        for word in text.split_whitespace() {
            collector.push(word);
        }
    } // the borrow `'out` ends here; `'text` goes on
    words.sort(); // `words` is ours again: we can mutate it
    words.dedup();
    println!("{} distinct words: {words:?}", words.len());
    // ANCHOR_END: main
}
