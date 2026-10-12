// harness: compile-fail E0597
//! Hypothetical rule B, "give every input and the output the SAME lifetime",
//! applied to `lookup` by hand. The body compiles; the valid caller does not.

pub struct Table {
    rows: Vec<(String, String)>,
}

// ANCHOR: rule_b
pub fn lookup<'a>(key: &'a str, table: &'a Table) -> Option<&'a str> {
    for (name, value) in &table.rows {
        if name == key {
            return Some(value.as_str());
        }
    }
    None
}
// ANCHOR_END: rule_b

// ANCHOR: caller
pub fn caller(table: &Table) {
    let hit;
    {
        let key = String::from("a"); // a temporary key...
        hit = lookup(&key, table);
    } // ...dropped here, but the result comes from `table`
    println!("{hit:?}");
}
// ANCHOR_END: caller
