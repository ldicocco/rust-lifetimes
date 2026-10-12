// harness: run
//! `Box<dyn Error>` means `Box<dyn Error + 'static>`. An error that borrows its
//! input needs `Box<dyn Error + '_>`, which ties the error to the input; an error
//! that owns its data fits the default and can travel anywhere.
//!
//! Run with: `cargo run --example ch08_error_box`

use std::error::Error;
use std::fmt;

// ANCHOR: borrowed
/// An error that points into the input: cheap, but tied to the input's lifetime.
#[derive(Debug)]
struct BadNumber<'a> {
    input: &'a str,
}

impl fmt::Display for BadNumber<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not a number: {:?}", self.input)
    }
}

impl Error for BadNumber<'_> {}

// `?` converts `BadNumber<'_>` into `Box<dyn Error + '_>`: the box may borrow `s`.
fn parse_port(s: &str) -> Result<u16, Box<dyn Error + '_>> {
    let port: u16 = s.trim().parse().map_err(|_| BadNumber { input: s })?;
    Ok(port)
}
// ANCHOR_END: borrowed

// ANCHOR: owned
/// The same error, owning a copy of the input: it satisfies `'static`.
#[derive(Debug)]
struct BadPort {
    input: String,
}

impl fmt::Display for BadPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not a port: {:?}", self.input)
    }
}

impl Error for BadPort {}

fn parse_port_owned(s: &str) -> Result<u16, Box<dyn Error>> {
    let port: u16 = s.trim().parse().map_err(|_| BadPort { input: s.into() })?;
    Ok(port)
}

// The owned error can leave the function that owns the text.
fn load_port() -> Result<u16, Box<dyn Error>> {
    let line = String::from(" 80x ");
    let port = parse_port_owned(&line)?;
    Ok(port)
}
// ANCHOR_END: owned

// ANCHOR: main
fn main() {
    let lines = [String::from("8080"), String::from("80x")];
    for line in &lines {
        match parse_port(line) {
            Ok(port) => println!("port {port}"),
            Err(e) => println!("error: {e}"),
        }
    }
    if let Err(e) = load_port() {
        println!("error: {e}");
    }
}
// ANCHOR_END: main
