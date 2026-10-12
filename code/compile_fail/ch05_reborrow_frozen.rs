// harness: compile-fail E0499
//! While a reborrow `&mut *words` is live, the original reference is frozen: using
//! it would create a second path to the same data.

// ANCHOR: frozen
pub fn collect(words: &mut Vec<String>) {
    let other = &mut *words;
    words.push(String::from("alpha"));
    other.push(String::from("beta"));
}
// ANCHOR_END: frozen
