// harness: compile-fail E0515
//! A function that creates its input cannot return an error that borrows it.

pub struct ParseError<'src> {
    pub line_no: usize,
    pub line: &'src str,
}

pub fn parse(src: &str) -> Result<usize, ParseError<'_>> {
    for (i, line) in src.lines().enumerate() {
        if !line.contains('=') {
            return Err(ParseError {
                line_no: i + 1,
                line,
            });
        }
    }
    Ok(src.lines().count())
}

fn read_source(name: &str) -> String {
    format!("# {name}\nkey = value")
}

// ANCHOR: load
pub fn load(name: &str) -> Result<usize, ParseError<'_>> {
    let text = read_source(name);
    parse(&text) // the error would borrow `text`, which dies here
}
// ANCHOR_END: load
