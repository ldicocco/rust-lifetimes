// harness: run
//! Version 3 of the log analyzer: the configuration owns its data and is
//! borrowed only during `run`; one lifetime parameter is left, named after what
//! it borrows.
//!
//! Run with: `cargo run --example ch14_refactor_v3`

use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Level {
    Info,
    Warn,
    Error,
}

fn parse_line(line: &str) -> Option<(Level, &str)> {
    let (level, message) = line.split_once(' ')?;
    let level = match level {
        "INFO" => Level::Info,
        "WARN" => Level::Warn,
        "ERROR" => Level::Error,
        _ => return None,
    };
    Some((level, message))
}

const LOG: &str = "\
INFO server started
WARN disk 91% full
INFO heartbeat
ERROR db timeout after 30s
WARN heartbeat late
ERROR disk full";

// ANCHOR: v3
/// Small, built once, read often: it owns its data.
struct Config {
    min_level: Level,
    ignore: Vec<String>,
}

/// Borrows the log it analyzes, and nothing else.
struct Analyzer<'src> {
    input: &'src str,
    counts: BTreeMap<Level, usize>,
    errors: Vec<&'src str>,
}

impl<'src> Analyzer<'src> {
    fn new(input: &'src str) -> Self {
        Analyzer {
            input,
            counts: BTreeMap::new(),
            errors: Vec::new(),
        }
    }

    /// The configuration is needed only during the call: borrow it there.
    fn run(&mut self, config: &Config) {
        for line in self.input.lines() {
            let Some((level, message)) = parse_line(line) else {
                continue;
            };
            let ignored = config
                .ignore
                .iter()
                .any(|word| message.contains(word.as_str()));
            if level < config.min_level || ignored {
                continue;
            }
            *self.counts.entry(level).or_insert(0) += 1;
            if level == Level::Error {
                self.errors.push(message);
            }
        }
    }

    fn counts(&self) -> &BTreeMap<Level, usize> {
        &self.counts
    }

    fn errors(&self) -> &[&'src str] {
        &self.errors
    }
}
// ANCHOR_END: v3

fn main() {
    // ANCHOR: main
    let input = String::from(LOG);
    let config = Config {
        min_level: Level::Warn,
        ignore: vec![String::from("heartbeat")],
    };
    let mut analyzer = Analyzer::new(&input);
    analyzer.run(&config);
    for (level, count) in analyzer.counts() {
        println!("{level:?}: {count}");
    }
    println!("errors: {:?}", analyzer.errors());
    // ANCHOR_END: main
}
