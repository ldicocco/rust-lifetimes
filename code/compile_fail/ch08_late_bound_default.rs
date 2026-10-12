// harness: compile-fail "lifetime may not live long enough"
//! The Reference says that `Box<dyn Source<'a>>` means `Box<dyn Source<'a> + 'a>`
//! when `Source<'a>: 'a`. In a function signature where `'a` is late-bound,
//! rustc 1.99 uses `'static` instead.

pub trait Source<'a>: 'a {
    fn text(&self) -> &'a str;
}

impl<'a> Source<'a> for &'a str {
    fn text(&self) -> &'a str {
        self
    }
}

// ANCHOR: late
pub fn boxed_source<'a>(text: &'a str) -> Box<dyn Source<'a>> {
    Box::new(text)
}
// ANCHOR_END: late
