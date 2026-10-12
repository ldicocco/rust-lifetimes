// harness: run
//! One lifetime parameter is enough as long as the borrows are used together;
//! a second one is needed when a result must outlive the shorter borrow.
//!
//! Run with: `cargo run --example ch02_two_lifetimes`

// ANCHOR: one
/// One lifetime for both fields: the compiler shrinks the longer borrow.
struct Entry<'a> {
    key: &'a str,
    value: &'a str,
}

fn describe(text: &str) {
    let key = String::from("host");
    // The value (from `text`, or the literal) outlives `key`; both fields
    // are used as `&'a str` for an `'a` that ends with this function.
    let entry = Entry {
        key: &key,
        value: text.lines().next().unwrap_or("unknown"),
    };
    println!("{} -> {}", entry.key, entry.value);
}
// ANCHOR_END: one

// ANCHOR: two
/// Two lifetimes: the result names `'t` only, so the pattern may die first.
struct Finder<'t, 'p> {
    text: &'t str,
    pattern: &'p str,
}

impl<'t, 'p> Finder<'t, 'p> {
    fn new(pattern: &'p str, text: &'t str) -> Self {
        Finder { text, pattern }
    }

    fn first_line_with(&self) -> Option<&'t str> {
        self.text.lines().find(|line| line.contains(self.pattern))
    }
}

fn main() {
    describe("example.org");
    let text = String::from("[server]\nport = 8080\nhost = example.org");
    let found;
    {
        let pattern = String::from("port");
        let finder = Finder::new(&pattern, &text);
        found = finder.first_line_with();
    } // `pattern` is dropped here
    println!("{found:?}"); // fine: `found` borrows `text` only
}
// ANCHOR_END: two
