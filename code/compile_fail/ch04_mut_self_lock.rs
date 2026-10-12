// harness: compile-fail E0499
//! `&'src mut Collector<'src>`: invariance pins `'src`, and the `&mut` borrow
//! must last for all of it, so the collector is borrowed for the rest of its life.

// ANCHOR: collector
pub struct Collector<'src> {
    words: Vec<&'src str>,
}

impl<'src> Collector<'src> {
    pub fn add(&'src mut self, line: &'src str) {
        self.words.extend(line.split_whitespace());
    }
}
// ANCHOR_END: collector

// ANCHOR: caller
pub fn caller() {
    let text = String::from("alpha beta");
    let more = String::from("gamma");
    let mut collector = Collector { words: Vec::new() };
    collector.add(&text);
    collector.add(&more);
}
// ANCHOR_END: caller
