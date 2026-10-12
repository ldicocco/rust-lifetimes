// harness: compile-fail E0597
//! A generic type with a `Drop` impl makes the drop check assume that `drop`
//! may use the borrows it holds.

// ANCHOR: log
pub struct Log<T> {
    entries: Vec<T>,
}

impl<T> Drop for Log<T> {
    fn drop(&mut self) {
        println!("dropping a log of {} entries", self.entries.len());
    }
}

pub fn record() {
    let mut log = Log {
        entries: Vec::new(),
    };
    let line = String::from("GET /index.html");
    log.entries.push(&line);
}
// ANCHOR_END: log
