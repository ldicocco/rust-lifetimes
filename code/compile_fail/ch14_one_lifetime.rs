// harness: compile-fail E0515
//! With a single lifetime parameter, the fields are tied to the shorter of the
//! line and the separator, so they cannot outlive a local separator.

// ANCHOR: one_lifetime
pub struct Fields<'a> {
    rest: Option<&'a str>,
    sep: &'a str, // the same lifetime as the line
}

impl<'a> Fields<'a> {
    pub fn new(line: &'a str, sep: &'a str) -> Self {
        Fields {
            rest: Some(line),
            sep,
        }
    }
}
// ANCHOR_END: one_lifetime

impl<'a> Iterator for Fields<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let rest = self.rest?;
        match rest.find(self.sep) {
            Some(i) => {
                self.rest = Some(&rest[i + self.sep.len()..]);
                Some(&rest[..i])
            }
            None => {
                self.rest = None;
                Some(rest)
            }
        }
    }
}

// ANCHOR: first_field
pub fn first_field(line: &str, sep: char) -> &str {
    let sep = sep.to_string();
    Fields::new(line, &sep).next().unwrap_or("")
}
// ANCHOR_END: first_field
