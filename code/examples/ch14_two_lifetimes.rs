// harness: run
//! A struct that borrows from two places needs two lifetime parameters when its
//! results borrow from only one of them.
//!
//! Run with: `cargo run --example ch14_two_lifetimes`

// ANCHOR: fields
/// Splits a line into fields at a separator. The fields are slices of the
/// line; the separator is only consulted.
struct Fields<'line, 'sep> {
    rest: Option<&'line str>,
    sep: &'sep str,
}

impl<'line, 'sep> Fields<'line, 'sep> {
    fn new(line: &'line str, sep: &'sep str) -> Self {
        assert!(!sep.is_empty(), "the separator must not be empty");
        Fields {
            rest: Some(line),
            sep,
        }
    }
}

impl<'line> Iterator for Fields<'line, '_> {
    type Item = &'line str; // fields borrow from the line only

    fn next(&mut self) -> Option<&'line str> {
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
// ANCHOR_END: fields

// ANCHOR: first_field
/// The separator is built inside the function and dropped at its end; the
/// result may borrow only from `line`.
fn first_field(line: &str, sep: char) -> &str {
    let sep = sep.to_string();
    Fields::new(line, &sep).next().unwrap_or("")
}
// ANCHOR_END: first_field

fn main() {
    // ANCHOR: main
    let record = "ada;lovelace;1815";
    let fields: Vec<&str> = Fields::new(record, ";").collect();
    println!("{fields:?}");
    println!("{:?}", first_field(record, ';'));
    // ANCHOR_END: main
}
