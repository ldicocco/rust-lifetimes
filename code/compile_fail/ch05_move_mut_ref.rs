// harness: compile-fail E0382
//! `let other = words;` with no type written moves the `&mut` reference: a
//! mutable reference is not `Copy`, and here the compiler has no reason to
//! reborrow.

fn add_word(words: &mut Vec<String>, word: &str) {
    words.push(word.to_owned());
}

// ANCHOR: moved
pub fn collect(words: &mut Vec<String>) {
    let other = words; // moves the reference out of `words`
    add_word(other, "alpha");
    add_word(words, "beta");
}
// ANCHOR_END: moved
