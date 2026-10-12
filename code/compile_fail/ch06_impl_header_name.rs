// harness: compile-fail E0261
//! The `'_` of an `impl` header cannot be referred to: a method that must
//! mention the struct's lifetime needs a named parameter on the `impl`.

pub struct Parser<'src> {
    input: &'src str,
    pos: usize,
}

// ANCHOR: unnamed
impl Parser<'_> {
    pub fn rest(&self) -> &'src str {
        &self.input[self.pos..]
    }
}
// ANCHOR_END: unnamed
