// harness: run
//! Two ways to write NLL problem case #3 so that stable Rust accepts it: the entry
//! API, and a lookup that does not keep its borrow on the "missing" path.
//!
//! Run with: `cargo run --example ch05_nll_workarounds`

use std::collections::HashMap;

// ANCHOR: entry
fn cached_entry<'m>(cache: &'m mut HashMap<String, String>, word: &str) -> &'m String {
    cache
        .entry(word.to_owned()) // allocates the key even on a hit
        .or_insert_with(|| word.to_uppercase())
}
// ANCHOR_END: entry

// ANCHOR: lookup_twice
fn cached_twice<'m>(cache: &'m mut HashMap<String, String>, word: &str) -> &'m String {
    if !cache.contains_key(word) {
        // `contains_key` returned a `bool`: no borrow survives the test.
        cache.insert(word.to_owned(), word.to_uppercase());
    }
    &cache[word] // a second lookup
}
// ANCHOR_END: lookup_twice

fn main() {
    let mut cache = HashMap::new();
    println!("{}", cached_entry(&mut cache, "alpha"));
    println!("{}", cached_entry(&mut cache, "alpha"));
    println!("{}", cached_twice(&mut cache, "beta"));
    println!("{}", cached_twice(&mut cache, "beta"));
    println!("{} entries", cache.len());
}
