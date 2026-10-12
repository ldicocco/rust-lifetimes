// harness: compile-fail E0597 edition=2021
//! Edition 2021: a temporary created in the tail expression of a block is dropped
//! *after* the block's local variables, so a temporary that borrows a local and
//! has a destructor outlives what it borrows. Edition 2024 accepts this code
//! (see `code/examples/ch05_temporaries_2024.rs`).

// ANCHOR: reader
/// A view of a text. It has a destructor, so dropping it counts as a use of
/// the borrow it holds.
pub struct Reader<'a> {
    text: &'a str,
}

impl<'a> Reader<'a> {
    pub fn new(text: &'a str) -> Self {
        Reader { text }
    }

    pub fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }
}

impl Drop for Reader<'_> {
    fn drop(&mut self) {}
}
// ANCHOR_END: reader

// ANCHOR: tail
pub fn count_words() -> usize {
    let text = String::from("one two three");
    Reader::new(&text).word_count()
}
// ANCHOR_END: tail
