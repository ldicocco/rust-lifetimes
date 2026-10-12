// harness: compile-fail E0499
//! Two accessor methods that each take `&mut self` borrow the whole value, even
//! though their bodies touch different fields: the caller sees only signatures.

pub struct Document {
    title: String,
    words: Vec<String>,
}

// ANCHOR: methods
impl Document {
    pub fn title_mut(&mut self) -> &mut String {
        &mut self.title
    }

    pub fn words_mut(&mut self) -> &mut Vec<String> {
        &mut self.words
    }
}

pub fn tag_as_draft(doc: &mut Document) {
    let title = doc.title_mut();
    let words = doc.words_mut();
    title.push_str(" (draft)");
    words.push(String::from("draft"));
}
// ANCHOR_END: methods
