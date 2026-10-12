// harness: run
//! An `Iterator` whose items borrow from the data the iterator reads, not from
//! the iterator: items can be kept, collected and outlive the iterator.
//!
//! Run with: `cargo run --example ch02_words_iterator`

// ANCHOR: slice_iter
fn two_at_once() {
    let ports = [8080, 8443, 9000];
    let mut iter = ports.iter(); // a `std::slice::Iter<'_, i32>`
    let first = iter.next(); // `Option<&i32>`, borrowing `ports`...
    let second = iter.next(); // ...so `iter` is free for another call
    println!("{first:?} {second:?}");
}
// ANCHOR_END: slice_iter

// ANCHOR: words
/// The words of a borrowed text.
struct Words<'src> {
    rest: &'src str,
}

impl<'src> Iterator for Words<'src> {
    type Item = &'src str; // one type for every call of `next`

    fn next(&mut self) -> Option<&'src str> {
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
// ANCHOR_END: words

// ANCHOR: main
fn main() {
    two_at_once();
    let text = String::from("lifetimes are regions not durations");
    // The `Words` value is a temporary, gone at the end of the statement;
    // the words it produced borrow `text` and remain usable.
    let long_words: Vec<&str> = Words { rest: &text }.filter(|w| w.len() > 3).collect();
    println!("{long_words:?}");
}
// ANCHOR_END: main
