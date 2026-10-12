// harness: run
//! Splitting borrows: two `&mut` borrows may coexist when the compiler can see
//! that they point to disjoint places (fields), or when an API guarantees it
//! (`split_at_mut`, `get_disjoint_mut`, a method that returns several fields).
//!
//! Run with: `cargo run --example ch05_split_borrows`

// ANCHOR: fields
struct Document {
    title: String,
    words: Vec<String>,
}

fn tag_as_draft(doc: &mut Document) {
    let title = &mut doc.title; // borrows only `doc.title`
    let words = &mut doc.words; // borrows only `doc.words`
    title.push_str(" (draft)");
    words.push(String::from("draft"));
}
// ANCHOR_END: fields

// ANCHOR: parts
impl Document {
    /// One method, one borrow of `self`, two disjoint results.
    fn parts_mut(&mut self) -> (&mut String, &mut Vec<String>) {
        (&mut self.title, &mut self.words)
    }
}

fn tag_as_final(doc: &mut Document) {
    let (title, words) = doc.parts_mut();
    title.push_str(" (final)");
    words.push(String::from("final"));
}
// ANCHOR_END: parts

// ANCHOR: destructure
fn tag_as_checked(doc: &mut Document) {
    let Document { title, words } = doc; // two `&mut` bindings, one per field
    title.push_str(" (checked)");
    words.push(String::from("checked"));
}
// ANCHOR_END: destructure

// ANCHOR: closures
fn tag_with_closures(doc: &mut Document) {
    // Each closure captures only the field it uses (edition 2021 and later).
    let mut retitle = |suffix: &str| doc.title.push_str(suffix);
    let mut add_word = |word: &str| doc.words.push(word.to_owned());
    retitle(" (reviewed)");
    add_word("reviewed");
}
// ANCHOR_END: closures

// ANCHOR: slices
fn swap_ends(words: &mut [String]) {
    let last = words.len() - 1;
    let (head, tail) = words.split_at_mut(1); // [0, 1) and [1, len)
    std::mem::swap(&mut head[0], &mut tail[last - 1]);
}

fn swap_first_two(words: &mut [String]) {
    // Checks at run time that the indices are distinct and in bounds.
    if let Ok([a, b]) = words.get_disjoint_mut([0, 1]) {
        std::mem::swap(a, b);
    }
}
// ANCHOR_END: slices

fn main() {
    let mut doc = Document {
        title: String::from("Notes"),
        words: Vec::new(),
    };
    tag_as_draft(&mut doc);
    tag_as_final(&mut doc);
    tag_with_closures(&mut doc);
    tag_as_checked(&mut doc);
    println!("{}: {:?}", doc.title, doc.words);

    swap_ends(&mut doc.words);
    println!("{:?}", doc.words);
    swap_first_two(&mut doc.words);
    println!("{:?}", doc.words);
}
