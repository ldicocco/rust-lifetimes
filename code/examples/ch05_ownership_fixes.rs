// harness: run
//! The usual fixes for E0505, E0515 and E0716: use the borrow before the move (or
//! own a copy of what you need), return owned data, and give a temporary a name.
//!
//! Run with: `cargo run --example ch05_ownership_fixes`

// ANCHOR: move_fix
fn parse_setting() -> (String, Vec<String>) {
    let line = String::from("timeout=30");
    let key = line.split('=').next().unwrap_or("").to_owned(); // an owned copy
    let mut lines = Vec::new();
    lines.push(line); // fine: `key` does not borrow `line`
    (key, lines)
}
// ANCHOR_END: move_fix

// ANCHOR: return_fix
fn first_word(path: &str) -> String {
    let text = format!("contents of {path}");
    text.split_whitespace().next().unwrap_or("").to_owned()
}
// ANCHOR_END: return_fix

// ANCHOR: temporary_fix
fn config_name() -> usize {
    let owned = String::from("config.toml"); // a named value lives to the block end
    let name: &str = owned.as_str();
    let extended: &String = &String::from("config.toml"); // lifetime-extended
    name.len() + extended.len()
}
// ANCHOR_END: temporary_fix

fn main() {
    let (key, lines) = parse_setting();
    println!("key = {key}, lines = {lines:?}");
    println!("first word = {}", first_word("app.toml"));
    println!("lengths = {}", config_name());
}
