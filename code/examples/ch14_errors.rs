// harness: run
//! Errors that borrow and errors that own. A parser can return an error that
//! borrows the offending line; a function that owns its input must return an
//! error that owns its data.
//!
//! Run with: `cargo run --example ch14_errors`

use std::error::Error;
use std::fmt;

// ANCHOR: borrowed_error
/// Cheap to create, but tied to the input it points into.
#[derive(Debug)]
struct ParseError<'src> {
    line_no: usize,
    line: &'src str,
}

/// Parses `key = value` lines; the pairs and the error borrow from `src`.
fn parse(src: &str) -> Result<Vec<(&str, &str)>, ParseError<'_>> {
    let mut pairs = Vec::new();
    for (i, line) in src.lines().enumerate() {
        match line.split_once('=') {
            Some((key, value)) => pairs.push((key.trim(), value.trim())),
            None => {
                return Err(ParseError {
                    line_no: i + 1,
                    line,
                });
            }
        }
    }
    Ok(pairs)
}
// ANCHOR_END: borrowed_error

// ANCHOR: owned_error
/// Owns its data: it can outlive the input, cross threads, and become a
/// `Box<dyn Error>` (which means `Box<dyn Error + 'static>`).
#[derive(Debug)]
struct ConfigError {
    source_name: String,
    line_no: usize,
    line: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (name, line_no, line) = (&self.source_name, self.line_no, &self.line);
        write!(f, "{name}:{line_no}: not a `key = value` line: {line:?}")
    }
}

impl Error for ConfigError {}
// ANCHOR_END: owned_error

/// Stands for reading a file: the text is created here, owned by the caller.
fn read_source(name: &str) -> String {
    match name {
        "good.conf" => String::from("name = indexer\nworkers = 4"),
        _ => String::from("name = indexer\nworkers 4"),
    }
}

// ANCHOR: load
/// Reads the text itself, so nothing it returns may borrow from that text.
fn load(name: &str) -> Result<Vec<(String, String)>, ConfigError> {
    let text = read_source(name);
    let pairs = parse(&text).map_err(|e| ConfigError {
        source_name: name.to_owned(),
        line_no: e.line_no,
        line: e.line.to_owned(),
    })?;
    Ok(pairs
        .into_iter()
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect())
}
// ANCHOR_END: load

fn main() {
    // ANCHOR: main
    // The caller owns the input: a borrowed error is fine here.
    let input = String::from("a = 1\nb 2");
    if let Err(e) = parse(&input) {
        println!("line {}: {:?}", e.line_no, e.line);
    }

    println!("{:?}", load("good.conf"));
    let error: Box<dyn Error> = load("bad.conf").unwrap_err().into();
    println!("{error}");
    // ANCHOR_END: main
}
