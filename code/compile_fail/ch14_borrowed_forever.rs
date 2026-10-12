// harness: compile-fail E0502
//! `&'text mut self` in `impl<'text> WordSet<'text>` borrows the set mutably for
//! `'text`, which lasts as long as the set is used: after the first call, the
//! set can never be used again.

// ANCHOR: forever
pub struct WordSet<'text> {
    words: Vec<&'text str>,
}

impl<'text> WordSet<'text> {
    pub fn new() -> Self {
        WordSet { words: Vec::new() }
    }

    pub fn add_all(&'text mut self, text: &'text str) {
        for word in text.split_whitespace() {
            if !self.words.contains(&word) {
                self.words.push(word);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.words.len()
    }
}

pub fn caller() -> usize {
    let text = String::from("the cat saw the dog");
    let mut set = WordSet::new();
    set.add_all(&text);
    set.len() // the set is still borrowed by `add_all`
}
// ANCHOR_END: forever
