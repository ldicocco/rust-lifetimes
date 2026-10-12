// Hypothetical rule B: "give every input and the output the SAME lifetime".
struct Table {
    rows: Vec<(String, String)>,
}

fn lookup<'a>(key: &'a str, table: &'a Table) -> Option<&'a str> {
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
