// harness: compile-fail E0502
//! Version 1 of a log analyzer: "lifetime soup". Three lifetime parameters, and
//! `&'a` on the receivers. The library compiles; its first caller does not.

use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
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

// ANCHOR: soup
pub struct Config<'c> {
    pub min_level: Level,
    pub ignore: Vec<&'c str>,
}

pub struct Analyzer<'a, 'b, 'c> {
    config: &'b Config<'c>,
    input: &'a str,
    counts: BTreeMap<Level, usize>,
    errors: Vec<&'a str>,
}

impl<'a, 'b, 'c> Analyzer<'a, 'b, 'c> {
    pub fn new(config: &'b Config<'c>, input: &'a str) -> Analyzer<'a, 'b, 'c> {
        Analyzer {
            config,
            input,
            counts: BTreeMap::new(),
            errors: Vec::new(),
        }
    }

    pub fn run(&'a mut self) {
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

    pub fn counts(&'a self) -> &'a BTreeMap<Level, usize> {
        &self.counts
    }

    pub fn errors(&'a self) -> &'a [&'a str] {
        &self.errors
    }
}
// ANCHOR_END: soup

// ANCHOR: caller
pub fn caller(input: &str) {
    let config = Config {
        min_level: Level::Warn,
        ignore: vec!["heartbeat"],
    };
    let mut analyzer = Analyzer::new(&config, input);
    analyzer.run();
    println!("{:?}", analyzer.counts());
}
// ANCHOR_END: caller
