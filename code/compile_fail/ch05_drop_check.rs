// harness: compile-fail E0597
//! The drop check: `report` is dropped after `text` (reverse order of
//! declaration), and its `Drop` impl could read the borrowed lines.

// ANCHOR: report
pub struct Report<'a> {
    lines: Vec<&'a str>,
}

impl Drop for Report<'_> {
    fn drop(&mut self) {
        println!("report: {} lines", self.lines.len());
    }
}
// ANCHOR_END: report

// ANCHOR: caller
pub fn summarize() {
    let mut report = Report { lines: Vec::new() };
    let text = String::from("first line\nsecond line");
    report.lines.extend(text.lines());
}
// ANCHOR_END: caller
