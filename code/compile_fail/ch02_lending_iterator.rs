// harness: compile-fail "declared without lifetime parameters"
//! An `Iterator` cannot hand out items that borrow from the iterator itself.

pub struct Normalizer<'src> {
    lines: std::str::Lines<'src>,
    buffer: String,
}

// ANCHOR: iterator
impl<'src> Iterator for Normalizer<'src> {
    type Item = &str; // which lifetime? not one of the iterator's parameters

    fn next(&mut self) -> Option<&str> {
        let raw = self.lines.next()?;
        self.buffer.clear();
        self.buffer.push_str(raw.trim());
        Some(&self.buffer)
    }
}
// ANCHOR_END: iterator
