// harness: compile-fail E0515
//! A function cannot return a reference to data it owns: the data is dropped
//! when the function returns. Two spellings, two wordings of the same error.

// ANCHOR: returned
pub fn first_word(path: &str) -> &str {
    let text = format!("contents of {path}");
    text.split_whitespace().next().unwrap_or("")
}

pub fn whole_text(path: &str) -> &String {
    let text = format!("contents of {path}");
    &text
}
// ANCHOR_END: returned
