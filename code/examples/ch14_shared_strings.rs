// harness: run
//! `Arc<str>` and `Rc<str>`: immutable strings with shared ownership. Cloning one
//! copies a pointer, the type has no lifetime parameter, and it is `'static`.
//!
//! Run with: `cargo run --example ch14_shared_strings`

use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

// ANCHOR: catalog
/// A file entry. Its name is also a key of the index below.
struct Entry {
    name: Arc<str>,
    size: u64,
}

/// Entries plus an index by name, sharing one allocation per name.
struct Catalog {
    entries: Vec<Entry>,
    by_name: BTreeMap<Arc<str>, usize>,
}

impl Catalog {
    fn new(files: &[(&str, u64)]) -> Self {
        let mut catalog = Catalog {
            entries: Vec::new(),
            by_name: BTreeMap::new(),
        };
        for &(name, size) in files {
            let name: Arc<str> = Arc::from(name); // one allocation
            let index = catalog.entries.len();
            catalog.by_name.insert(Arc::clone(&name), index); // a second owner
            catalog.entries.push(Entry { name, size });
        }
        catalog
    }

    fn size_of(&self, name: &str) -> Option<u64> {
        // `Arc<str>: Borrow<str>`, so a `&str` is enough to look up a key.
        self.by_name.get(name).map(|&i| self.entries[i].size)
    }
}
// ANCHOR_END: catalog

fn main() {
    // ANCHOR: main
    let catalog = Catalog::new(&[("notes.txt", 120), ("todo.md", 30)]);
    println!("todo.md: {:?} bytes", catalog.size_of("todo.md"));
    let first = &catalog.entries[0].name;
    println!("{first}: {} owners", Arc::strong_count(first)); // entry + index

    // No lifetime parameter, and `'static`: the names can go to a thread.
    let names: Vec<Arc<str>> = catalog.entries.iter().map(|e| e.name.clone()).collect();
    let handle = thread::spawn(move || names.join(", "));
    println!("from a thread: {}", handle.join().unwrap());

    // `Rc<str>` is the same idea for data that stays on one thread.
    let tag: Rc<str> = Rc::from("draft");
    let tags = [Rc::clone(&tag), Rc::clone(&tag)];
    println!("{}: {} owners", tags[1], Rc::strong_count(&tag));
    // ANCHOR_END: main
}
