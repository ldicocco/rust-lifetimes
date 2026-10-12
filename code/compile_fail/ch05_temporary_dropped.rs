// harness: compile-fail E0716
//! A temporary lives until the end of the statement that creates it, unless it
//! is lifetime-extended; extension does not go through a method call or a
//! function call. Two functions, one error each.

// ANCHOR: temporary
pub fn config_name() -> usize {
    let name: &str = String::from("config.toml").as_str();
    name.len()
}
// ANCHOR_END: temporary

// ANCHOR: call
pub fn config_path() -> usize {
    let path = std::convert::identity(&String::from("/etc/app"));
    path.len()
}
// ANCHOR_END: call
