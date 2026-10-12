// harness: compile-fail E0502
//! The "borrowed forever" shape in a field: `&'a mut Vec<&'a str>` ties the
//! mutable borrow of `words` to the lifetime of the strings inside it.

// ANCHOR: collector
/// Pushes words into a vector owned by someone else.
pub struct Collector<'a> {
    out: &'a mut Vec<&'a str>,
}

impl<'a> Collector<'a> {
    pub fn push(&mut self, word: &'a str) {
        self.out.push(word);
    }
}

pub fn caller() -> usize {
    let text = String::from("to be or not to be");
    let mut words: Vec<&str> = Vec::new();
    {
        let mut collector = Collector { out: &mut words };
        for word in text.split_whitespace() {
            collector.push(word);
        }
    } // the collector is gone, but `words` is still mutably borrowed
    words.len()
}
// ANCHOR_END: collector
