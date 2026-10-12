// harness: compile-fail E0597 "lifetime may not live long enough"
//! Hypothetical rule A, "tie the output to the FIRST reference parameter",
//! applied to `lookup` by hand. It rejects the body and the caller.

pub struct Table {
    rows: Vec<(String, String)>,
}

// ANCHOR: rule_a
pub fn lookup<'k, 't>(key: &'k str, table: &'t Table) -> Option<&'k str> {
    for (name, value) in &table.rows {
        if name == key {
            return Some(value.as_str());
        }
    }
    None
}
// ANCHOR_END: rule_a

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
