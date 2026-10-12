// harness: run
//! Lazily initialized globals hand out `&'static` references: `OnceLock` (you
//! choose when and how to initialize) and `LazyLock` (the initializer is part of
//! the declaration).
//!
//! Run with: `cargo run --example ch07_globals`

use std::collections::BTreeMap;
use std::sync::{LazyLock, OnceLock};

#[derive(Debug)]
struct Config {
    name: String,
    workers: usize,
}

// ANCHOR: oncelock
static CONFIG: OnceLock<Config> = OnceLock::new();

/// Initializes the configuration once; later calls return the same value.
fn config() -> &'static Config {
    CONFIG.get_or_init(|| {
        println!("(loading config)");
        Config {
            name: String::from("indexer"),
            workers: 4,
        }
    })
}
// ANCHOR_END: oncelock

// ANCHOR: lazylock
static KEYWORDS: LazyLock<BTreeMap<&'static str, usize>> = LazyLock::new(|| {
    println!("(building keyword table)");
    ["fn", "let", "match", "static"]
        .into_iter()
        .enumerate()
        .map(|(id, word)| (word, id))
        .collect()
});

fn keyword_id(word: &str) -> Option<usize> {
    KEYWORDS.get(word).copied()
}
// ANCHOR_END: lazylock

fn main() {
    // ANCHOR: main
    let first: &'static Config = config();
    let second: &'static Config = config(); // no second "(loading config)"
    assert!(std::ptr::eq(first, second));
    println!("{} with {} workers", first.name, second.workers);

    println!("static -> {:?}", keyword_id("static"));
    println!("struct -> {:?}", keyword_id("struct"));
    // ANCHOR_END: main
}
