// harness: compile-fail E0308
//! `&String` coerces to `&str`, but a coercion converts one value at one place;
//! unlike subtyping, it does not pass through `Vec`.

// ANCHOR: names
fn total_len(names: Vec<&str>) -> usize {
    names.iter().map(|name| name.len()).sum()
}

pub fn caller() -> usize {
    let owned = String::from("ferris");
    let one: &str = &owned; // fine: a deref coercion from &String to &str
    let names: Vec<&String> = vec![&owned];
    total_len(names) + one.len()
}
// ANCHOR_END: names
