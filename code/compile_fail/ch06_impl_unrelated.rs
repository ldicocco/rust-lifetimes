// harness: compile-fail "lifetime may not live long enough"
//! Following the second suggestion of E0261 literally: `impl<'src> Parser<'_>`
//! declares a `'src` that has nothing to do with the parser's lifetime.

pub struct Parser<'src> {
    input: &'src str,
    pos: usize,
}

// ANCHOR: unrelated
impl<'src> Parser<'_> {
    pub fn rest(&self) -> &'src str {
        &self.input[self.pos..]
    }
}
// ANCHOR_END: unrelated
