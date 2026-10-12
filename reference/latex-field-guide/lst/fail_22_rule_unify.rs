// Hypothetical rule B: "give every input and the output the SAME lifetime".
// struct Table and main(): identical to the previous listing

fn lookup<'a>(key: &'a str, table: &'a Table) -> Option<&'a str> {
    table.rows.iter().find(|row| row.0 == key).map(|row| row.1.as_str())
}
