// harness: compile-fail E0499
//! The same error as with the elided parser, but this time the compiler
//! protects you: the second call overwrites the buffer that `first` points to.

pub struct Normalizer<'src> {
    lines: std::str::Lines<'src>,
    buffer: String,
}

impl<'src> Normalizer<'src> {
    pub fn next_line(&mut self) -> Option<&str> {
        let raw = self.lines.next()?;
        self.buffer.clear();
        self.buffer.push_str(raw.trim());
        Some(&self.buffer)
    }
}

// ANCHOR: keep
pub fn caller(normalizer: &mut Normalizer<'_>) {
    let first = normalizer.next_line();
    let second = normalizer.next_line(); // would overwrite what `first` shows
    println!("{first:?} {second:?}");
}
// ANCHOR_END: keep
