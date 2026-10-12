// harness: run
//! Version 4 of the log analyzer: a borrowed view for the short-lived work
//! (`Entry<'src>`), an owned result for what is kept (`Report`), and a function
//! as the boundary. No long-lived type has a lifetime parameter.
//!
//! Run with: `cargo run --example ch14_refactor_v4`

use std::collections::BTreeMap;
use std::thread;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Level {
    Info,
    Warn,
    Error,
}

const LOG: &str = "\
INFO server started
WARN disk 91% full
INFO heartbeat
ERROR db timeout after 30s
WARN heartbeat late
ERROR disk full";

// ANCHOR: entry
/// One parsed line: a zero-copy view into the log.
#[derive(Clone, Copy, Debug)]
struct Entry<'src> {
    level: Level,
    message: &'src str,
}

impl<'src> Entry<'src> {
    fn parse(line: &'src str) -> Option<Self> {
        let (level, message) = line.split_once(' ')?;
        let level = match level {
            "INFO" => Level::Info,
            "WARN" => Level::Warn,
            "ERROR" => Level::Error,
            _ => return None,
        };
        Some(Entry { level, message })
    }
}

/// The entries of a log, borrowed from it.
fn entries(input: &str) -> impl Iterator<Item = Entry<'_>> {
    input.lines().filter_map(Entry::parse)
}
// ANCHOR_END: entry

// ANCHOR: report
struct Config {
    min_level: Level,
    ignore: Vec<String>,
}

impl Config {
    fn accepts(&self, entry: &Entry<'_>) -> bool {
        let ignored = self
            .ignore
            .iter()
            .any(|word| entry.message.contains(word.as_str()));
        entry.level >= self.min_level && !ignored
    }
}

/// What we keep: it owns its data, so it can outlive the log.
#[derive(Default)]
struct Report {
    counts: BTreeMap<Level, usize>,
    errors: Vec<String>,
}

/// Borrows the log and the configuration for the duration of the call only.
fn analyze(input: &str, config: &Config) -> Report {
    let mut report = Report::default();
    for entry in entries(input).filter(|entry| config.accepts(entry)) {
        *report.counts.entry(entry.level).or_insert(0) += 1;
        if entry.level == Level::Error {
            report.errors.push(entry.message.to_owned());
        }
    }
    report
}
// ANCHOR_END: report

fn main() {
    // ANCHOR: main
    let input = String::from(LOG);
    let config = Config {
        min_level: Level::Warn,
        ignore: vec![String::from("heartbeat")],
    };
    let report = analyze(&input, &config);
    drop(input); // the report does not borrow the log
    for (level, count) in &report.counts {
        println!("{level:?}: {count}");
    }
    println!("errors: {:?}", report.errors);

    // `Report: Send + 'static`: it can be handed to another thread.
    let handle = thread::spawn(move || report.errors.len());
    assert_eq!(handle.join().unwrap(), 2);
    // ANCHOR_END: main
}
