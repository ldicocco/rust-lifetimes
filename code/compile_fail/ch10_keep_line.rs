// harness: compile-fail E0499
//! Keeping an item of a lending iterator past the next call to `next` keeps
//! the iterator borrowed into the next iteration of the loop.

pub trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

pub struct TrimmedLines<'src> {
    lines: std::str::Lines<'src>,
    buffer: String,
}

impl<'src> LendingIterator for TrimmedLines<'src> {
    type Item<'a>
        = &'a str
    where
        Self: 'a;

    fn next(&mut self) -> Option<&str> {
        let line = self.lines.next()?;
        self.buffer.clear();
        self.buffer.push_str(line.trim());
        Some(&self.buffer)
    }
}

// ANCHOR: keep
pub fn keep_all(mut lines: TrimmedLines<'_>) -> usize {
    let mut keys = Vec::new();
    while let Some(line) = lines.next() {
        keys.push(line); // keeps a borrow of `lines.buffer`
    }
    keys.len()
}
// ANCHOR_END: keep
