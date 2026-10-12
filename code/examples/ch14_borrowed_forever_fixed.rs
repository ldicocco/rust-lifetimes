// harness: run
//! The fix for "borrowed forever": the borrow of `self` gets its own, short,
//! anonymous lifetime; only the text is borrowed for `'text`.
//!
//! Run with: `cargo run --example ch14_borrowed_forever_fixed`

// ANCHOR: fixed
struct WordSet<'text> {
    words: Vec<&'text str>,
}

impl<'text> WordSet<'text> {
    fn new() -> Self {
        WordSet { words: Vec::new() }
    }

    fn add_all(&mut self, text: &'text str) {
        for word in text.split_whitespace() {
            if !self.words.contains(&word) {
                self.words.push(word);
            }
        }
    }

    fn len(&self) -> usize {
        self.words.len()
    }
}
// ANCHOR_END: fixed

fn main() {
    // ANCHOR: main
    let text = String::from("the cat saw the dog");
    let mut set = WordSet::new();
    set.add_all(&text);
    println!("{} distinct words", set.len());
    set.add_all("the end"); // a second mutable borrow: fine now
    println!("{} distinct words: {:?}", set.len(), set.words);
    // ANCHOR_END: main
}
