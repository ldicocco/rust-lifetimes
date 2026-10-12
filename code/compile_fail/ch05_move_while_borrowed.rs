// harness: compile-fail E0505
//! A value cannot be moved while a borrow of it is still going to be used: the
//! move could take the data somewhere the reference does not point.

// ANCHOR: moved
pub fn parse_setting() -> Vec<String> {
    let line = String::from("timeout=30");
    let key = line.split('=').next().unwrap_or("");
    let mut lines = Vec::new();
    lines.push(line); // moves `line` into the vector
    println!("key: {key}");
    lines
}
// ANCHOR_END: moved
