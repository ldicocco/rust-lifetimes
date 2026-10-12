// Hypothetical rule A: "tie the output to the FIRST reference parameter".
struct Table {
    rows: Vec<(String, String)>,
}

fn lookup<'k, 't>(key: &'k str, table: &'t Table) -> Option<&'k str> {
    table.rows.iter().find(|row| row.0 == key).map(|row| row.1.as_str())
}

fn main() {
    let table = Table { rows: vec![("a".to_string(), "alpha".to_string())] };
    let hit;
    {
        let key = String::from("a"); // a temporary key ...
        hit = lookup(&key, &table);
    } // ... dies here, but the result comes from `table`
    println!("{hit:?}");
}
