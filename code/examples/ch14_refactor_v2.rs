// harness: run
//! Version 2 of the log analyzer: the receivers no longer borrow the analyzer
//! for `'a`. It compiles, but it still has three lifetime parameters.
//!
//! Run with: `cargo run --example ch14_refactor_v2`

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

struct Config<'c> {
    min_level: Level,
    ignore: Vec<&'c str>,
}

struct Analyzer<'a, 'b, 'c> {
    config: &'b Config<'c>,
    input: &'a str,
    counts: BTreeMap<Level, usize>,
    errors: Vec<&'a str>,
}

impl<'a, 'b, 'c> Analyzer<'a, 'b, 'c> {
    fn new(config: &'b Config<'c>, input: &'a str) -> Analyzer<'a, 'b, 'c> {
        Analyzer {
            config,
            input,
            counts: BTreeMap::new(),
            errors: Vec::new(),
        }
    }

    fn run(&mut self) {
        for line in self.input.lines() {
            let Some((level, message)) = parse_line(line) else {
                continue;
            };
            let ignored = self.config.ignore.iter().any(|word| message.contains(word));
            if level < self.config.min_level || ignored {
                continue;
            }
            *self.counts.entry(level).or_insert(0) += 1;
            if level == Level::Error {
                self.errors.push(message);
            }
        }
    }

    // ANCHOR: getters
    // `run` is now `fn run(&mut self)`: a short borrow, chosen at each call.
    fn counts(&self) -> &BTreeMap<Level, usize> {
        &self.counts
    }

    fn errors(&self) -> &[&'a str] {
        &self.errors
    }
    // ANCHOR_END: getters
}

fn main() {
    // ANCHOR: main
    let input = String::from(LOG);
    let config = Config {
        min_level: Level::Warn,
        ignore: vec!["heartbeat"],
    };
    let mut analyzer = Analyzer::new(&config, &input);
    analyzer.run();
    for (level, count) in analyzer.counts() {
        println!("{level:?}: {count}");
    }
    println!("errors: {:?}", analyzer.errors());
    // ANCHOR_END: main
}
