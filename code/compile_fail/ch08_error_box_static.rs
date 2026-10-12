// harness: compile-fail "lifetime may not live long enough"
//! `?` converts the error with `From`, into `Box<dyn Error>`, which means
//! `Box<dyn Error + 'static>`: an error that borrows `s` does not fit.

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

// ANCHOR: parse
pub fn parse_port(s: &str) -> Result<u16, Box<dyn Error>> {
    let port: u16 = s.trim().parse().map_err(|_| BadNumber { input: s })?;
    Ok(port)
}
// ANCHOR_END: parse
