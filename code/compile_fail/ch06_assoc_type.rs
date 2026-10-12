// harness: compile-fail E0637
//! An associated type in an `impl` cannot use `'_`: it needs a lifetime declared
//! on the `impl`.

pub struct Words<'src> {
    rest: &'src str,
}

// ANCHOR: assoc
impl Iterator for Words<'_> {
    type Item = &'_ str;

    fn next(&mut self) -> Option<Self::Item> {
        let trimmed = self.rest.trim_start();
        let end = trimmed.find(' ').unwrap_or(trimmed.len());
        let (word, rest) = trimmed.split_at(end);
        self.rest = rest;
        (!word.is_empty()).then_some(word)
    }
}
// ANCHOR_END: assoc
