// harness: compile-fail E0515 E0505
//! A struct cannot hold both a `String` and a borrow of that same string: the
//! constructor would return a value that borrows its own local.

// ANCHOR: document
pub struct Document<'a> {
    text: String,
    title: &'a str, // meant to point into `text`
}

impl<'a> Document<'a> {
    pub fn new(text: String) -> Document<'a> {
        let title = text.lines().next().unwrap_or("");
        Document { text, title }
    }
}
// ANCHOR_END: document
