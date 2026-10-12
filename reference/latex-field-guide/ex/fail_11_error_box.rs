use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct BadInput<'a> {
    input: &'a str,
}

impl fmt::Display for BadInput<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bad input: {:?}", self.input)
    }
}

impl Error for BadInput<'_> {}

fn parse(s: &str) -> Result<i32, Box<dyn Error>> {
    s.parse::<i32>().map_err(|_| BadInput { input: s }.into())
}

fn main() {
    println!("{:?}", parse("12x").is_err());
}
