// harness: run
//! A "lending" reader: every line it returns borrows from a buffer inside the
//! reader, which the next call overwrites. Here the elided return type is the
//! right contract.
//!
//! Run with: `cargo run --example ch02_lending`

// ANCHOR: normalizer
/// Yields the non-blank lines of a text, lowercased, with single spaces.
struct Normalizer<'src> {
    lines: std::str::Lines<'src>, // borrows the text
    buffer: String,               // owned, reused for every line
}

impl<'src> Normalizer<'src> {
    fn new(text: &'src str) -> Self {
        Normalizer {
            lines: text.lines(),
            buffer: String::new(),
        }
    }

    /// The line borrows from `self.buffer`, which the next call overwrites.
    fn next_line(&mut self) -> Option<&str> {
        loop {
            let raw = self.lines.next()?;
            if raw.trim().is_empty() {
                continue;
            }
            self.buffer.clear();
            for (i, word) in raw.split_whitespace().enumerate() {
                if i > 0 {
                    self.buffer.push(' ');
                }
                self.buffer.push_str(&word.to_lowercase());
            }
            return Some(&self.buffer);
        }
    }
}
// ANCHOR_END: normalizer

// ANCHOR: main
fn main() {
    let text = String::from("  Port =   8080\n\n HOST = Example.ORG \n");
    let mut normalizer = Normalizer::new(&text);
    let mut kept: Vec<String> = Vec::new();
    while let Some(line) = normalizer.next_line() {
        println!("{line:?}");
        kept.push(line.to_owned()); // to keep a line, copy it out of the buffer
    }
    println!("{kept:?}");
}
// ANCHOR_END: main
