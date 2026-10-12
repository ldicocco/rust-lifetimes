// harness: compile-fail E0106
//! Elision is syntactic: `self` has the type `Self`, which is `Wrapper<'a>`, but
//! no lifetime is WRITTEN in the receiver, so there is nothing to elide from.

// ANCHOR: by_value
pub struct Wrapper<'a> {
    text: &'a str,
}

impl<'a> Wrapper<'a> {
    pub fn into_text(self) -> &str {
        self.text
    }
}
// ANCHOR_END: by_value
