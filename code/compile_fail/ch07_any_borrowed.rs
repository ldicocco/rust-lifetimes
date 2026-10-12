// harness: compile-fail "lifetime may not live long enough"
//! `Any` is only implemented for `'static` types, so a type that borrows cannot
//! become a `Box<dyn Any>`.

use std::any::Any;

// ANCHOR: erase
/// A token that borrows its text from the source it was read from.
pub struct Token<'src> {
    pub text: &'src str,
}

pub fn erase<'src>(token: Token<'src>) -> Box<dyn Any> {
    Box::new(token)
}
// ANCHOR_END: erase
