// Hypothetical rule A: "tie the output to the FIRST reference parameter".
// struct Table and main(): identical to the previous listing

fn lookup<'k, 't>(key: &'k str, table: &'t Table) -> Option<&'k str> {
    table.rows.iter().find(|row| row.0 == key).map(|row| row.1.as_str())
}
