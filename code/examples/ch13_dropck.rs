// harness: run
//! `Vec<&T>` may hold borrows that die just before it, because `Vec`'s `Drop`
//! promises (with the unstable `#[may_dangle]`) not to use them. A type of your
//! own can get the same effect by keeping its `Drop` away from `T`.
//!
//! Run with: `cargo run --example ch13_dropck`

// ANCHOR: vec
fn record_in_vec() -> usize {
    let mut entries = Vec::new();
    let line = String::from("GET /index.html");
    entries.push(&line);
    entries.len()
} // `line` is dropped first, then `entries`: accepted
// ANCHOR_END: vec

// ANCHOR: log
/// The `Drop` lives in a field that holds no `T`.
struct Report {
    count: usize,
}

impl Drop for Report {
    fn drop(&mut self) {
        println!("dropping a log of {} entries", self.count);
    }
}

pub struct Log<T> {
    entries: Vec<T>,
    report: Report,
}

impl<T> Log<T> {
    pub fn new() -> Self {
        Log {
            entries: Vec::new(),
            report: Report { count: 0 },
        }
    }

    pub fn push(&mut self, entry: T) {
        self.entries.push(entry);
        self.report.count = self.entries.len();
    }
}

fn record_in_log() {
    let mut log = Log::new();
    let line = String::from("GET /index.html");
    log.push(&line);
} // `Log<&String>` has no `Drop` impl of its own: accepted
// ANCHOR_END: log

fn main() {
    println!("vec: {} entries", record_in_vec());
    record_in_log();
}
