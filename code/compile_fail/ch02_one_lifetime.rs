// harness: compile-fail E0597
//! One lifetime parameter for two unrelated borrows: the result of
//! `first_line_with` is limited by the shorter of the two.

// ANCHOR: finder
pub struct Finder<'a> {
    text: &'a str,
    pattern: &'a str,
}

impl<'a> Finder<'a> {
    pub fn new(pattern: &'a str, text: &'a str) -> Self {
        Finder { text, pattern }
    }

    pub fn first_line_with(&self) -> Option<&'a str> {
        self.text.lines().find(|line| line.contains(self.pattern))
    }
}
// ANCHOR_END: finder

// ANCHOR: caller
pub fn caller() {
    let text = String::from("[server]\nport = 8080\nhost = example.org");
    let found;
    {
        let pattern = String::from("port");
        let finder = Finder::new(&pattern, &text);
        found = finder.first_line_with();
    } // `pattern` is dropped here
    println!("{found:?}");
}
// ANCHOR_END: caller
