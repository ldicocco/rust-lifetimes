// harness: run
//! The intended contract of `lookup`, written explicitly: the result borrows from
//! `table`, not from `key`, so the key may be a short-lived temporary.
//!
//! Run with: `cargo run --example ch03_lookup_explicit`

// ANCHOR: lookup
struct Table {
    rows: Vec<(String, String)>,
}

fn lookup<'t>(key: &str, table: &'t Table) -> Option<&'t str> {
    for (name, value) in &table.rows {
        if name == key {
            return Some(value.as_str());
        }
    }
    None
}
// ANCHOR_END: lookup

// ANCHOR: caller
fn main() {
    let table = Table {
        rows: vec![("a".to_string(), "alpha".to_string())],
    };
    let hit;
    {
        let key = String::from("a"); // a temporary key...
        hit = lookup(&key, &table);
    } // ...dropped here, but the result comes from `table`
    println!("{hit:?}");
}
// ANCHOR_END: caller
