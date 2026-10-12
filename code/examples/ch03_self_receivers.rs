// harness: run
//! Rule 3: a method's elided output borrows from `self`. Since Rust 1.81 this
//! also works for receivers such as `self: &Rc<Self>` and `self: &Box<Self>`.
//!
//! Run with: `cargo run --example ch03_self_receivers`

use std::pin::Pin;
use std::rc::Rc;

struct Config {
    name: String,
    entries: Vec<(String, String)>,
}

impl Config {
    // ANCHOR: ref_self
    // Two inputs with lifetimes, but rule 3 picks `&self`:
    // fn get<'s, 'k>(&'s self, key: &'k str) -> Option<&'s str>
    fn get(&self, key: &str) -> Option<&str> {
        let entry = self.entries.iter().find(|(k, _)| k == key);
        entry.map(|(_, value)| value.as_str())
    }
    // ANCHOR_END: ref_self

    // ANCHOR: receivers
    // The reference points to a type that contains `Self` (Rust 1.81 and later).
    fn name_shared(self: &Rc<Self>, _requested_by: &str) -> &str {
        &self.name
    }

    fn name_boxed(self: &Box<Self>, _requested_by: &str) -> &str {
        &self.name
    }

    // The reference points directly to `Self`, inside `Pin`.
    fn name_pinned(self: Pin<&mut Self>, _requested_by: &str) -> &str {
        &self.get_mut().name
    }
    // ANCHOR_END: receivers

    // ANCHOR: by_value
    // A receiver without a reference lends nothing, so rule 3 does not apply and
    // rule 2 looks at the other parameters: the result borrows from `line`.
    fn strip_name(self: Box<Self>, line: &str) -> &str {
        line.strip_prefix(self.name.as_str()).unwrap_or(line)
    }
    // ANCHOR_END: by_value
}

fn main() {
    // ANCHOR: caller
    let config = Config {
        name: String::from("server"),
        entries: vec![(String::from("mode"), String::from("fast"))],
    };
    let mode;
    {
        let key = String::from("mode"); // a temporary key...
        mode = config.get(&key);
    } // ...dropped here: fine, the result borrows from `config` only
    println!("mode = {mode:?}");
    // ANCHOR_END: caller

    let shared = Rc::new(Config {
        name: String::from("shared"),
        entries: Vec::new(),
    });
    println!("{}", shared.name_shared("main"));

    let boxed = Box::new(Config {
        name: String::from("boxed"),
        entries: Vec::new(),
    });
    println!("{}", boxed.name_boxed("main"));

    let mut pinned = Config {
        name: String::from("pinned"),
        entries: Vec::new(),
    };
    println!("{}", Pin::new(&mut pinned).name_pinned("main"));

    let stripped = boxed.strip_name("boxed: ready");
    println!("{stripped}");
}
