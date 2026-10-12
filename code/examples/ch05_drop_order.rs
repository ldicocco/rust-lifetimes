// harness: run
//! Drop order: local variables in reverse order of declaration, struct fields in
//! declaration order, `let _ = ...` immediately. A type with a `Drop` impl that
//! holds a borrow must be declared after the data it borrows; a `Vec<&str>`, or a
//! struct without a `Drop` impl, need not be.
//!
//! Run with: `cargo run --example ch05_drop_order`

struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  drop {}", self.0);
    }
}

// ANCHOR: pair
struct Pair {
    first: Noisy,
    second: Noisy,
}
// ANCHOR_END: pair

// ANCHOR: report
/// A report that prints a summary when it goes away: its destructor reads the
/// borrowed lines.
struct Report<'a> {
    lines: Vec<&'a str>,
}

impl Drop for Report<'_> {
    fn drop(&mut self) {
        let first = self.lines.first().copied().unwrap_or("");
        println!("  report: {} lines, first {first:?}", self.lines.len());
    }
}
// ANCHOR_END: report

// ANCHOR: notes
/// The same fields, but no `Drop` impl.
struct Notes<'a> {
    lines: Vec<&'a str>,
}
// ANCHOR_END: notes

fn main() {
    // ANCHOR: order
    println!("locals and fields:");
    {
        let _a = Noisy("a");
        let _b = Noisy("b");
        let pair = Pair {
            first: Noisy("pair.first"),
            second: Noisy("pair.second"),
        };
        let _ = Noisy("ignored"); // `_` is not a binding: dropped right away
        println!("  end of block ({} and {})", pair.first.0, pair.second.0);
    }
    // ANCHOR_END: order

    // ANCHOR: report_ok
    println!("a report declared after its text:");
    {
        let text = String::from("first line\nsecond line");
        let mut report = Report { lines: Vec::new() };
        report.lines.extend(text.lines());
    } // `report` is dropped first, while `text` is still alive
    // ANCHOR_END: report_ok

    // ANCHOR: plain_vec
    println!("a plain Vec<&str> declared before its text:");
    {
        let mut lines: Vec<&str> = Vec::new();
        let text = String::from("first line\nsecond line");
        lines.extend(text.lines());
        println!("  {} lines", lines.len());
    } // `text` is dropped first; `Vec`'s destructor never reads the `&str`s
    // ANCHOR_END: plain_vec

    // ANCHOR: no_drop
    println!("a struct without Drop declared before its text:");
    {
        let mut notes = Notes { lines: Vec::new() };
        let text = String::from("first line\nsecond line");
        notes.lines.extend(text.lines());
        println!("  {} lines", notes.lines.len());
    } // no `impl Drop for Notes`: dropping `notes` reads nothing
    // ANCHOR_END: no_drop
}
