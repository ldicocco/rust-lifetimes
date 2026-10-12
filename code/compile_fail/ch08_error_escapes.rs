// harness: compile-fail E0515
//! With `Box<dyn Error + '_>` the error borrows its input, so it cannot be
//! propagated out of the function that owns the input.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct BadNumber<'a> {
    input: &'a str,
}

impl fmt::Display for BadNumber<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not a number: {:?}", self.input)
    }
}

impl Error for BadNumber<'_> {}

pub fn parse_port(s: &str) -> Result<u16, Box<dyn Error + '_>> {
    let port: u16 = s.trim().parse().map_err(|_| BadNumber { input: s })?;
    Ok(port)
}

// ANCHOR: escapes
pub fn load_port() -> Result<u16, Box<dyn Error>> {
    let line = String::from(" 80x ");
    let port = parse_port(&line)?;
    Ok(port)
}
// ANCHOR_END: escapes
