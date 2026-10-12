// harness: compile-fail E0597
//! Dropping `report` explicitly does not help: if `extend` panicked, unwinding
//! would drop `text` and then `report`, whose destructor reads the lines.

pub struct Report<'a> {
    lines: Vec<&'a str>,
}

impl Drop for Report<'_> {
    fn drop(&mut self) {
        println!("report: {} lines", self.lines.len());
    }
}

// ANCHOR: caller
pub fn summarize() {
    let mut report = Report { lines: Vec::new() };
    let text = String::from("first line\nsecond line");
    report.lines.extend(text.lines()); // may panic (allocation)
    drop(report);
}
// ANCHOR_END: caller
