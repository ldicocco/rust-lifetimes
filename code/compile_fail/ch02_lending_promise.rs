// harness: compile-fail "lifetime may not live long enough"
//! Promising `'src` for a line that lives in the reader's own buffer: the body
//! breaks the signature, and the compiler says so.

pub struct Normalizer<'src> {
    lines: std::str::Lines<'src>,
    buffer: String,
}

// ANCHOR: promise
impl<'src> Normalizer<'src> {
    pub fn next_line(&mut self) -> Option<&'src str> {
        let raw = self.lines.next()?;
        self.buffer.clear();
        self.buffer.push_str(raw.trim());
        Some(&self.buffer) // borrows `self`, not the text
    }
}
// ANCHOR_END: promise
