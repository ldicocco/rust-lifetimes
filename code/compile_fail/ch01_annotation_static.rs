// harness: compile-fail E0597
//! An annotation cannot make a borrow last longer than its referent: it only
//! adds a requirement, which here cannot be met.

// ANCHOR: annotation
pub fn caller() {
    let name = String::from("config.toml");
    let forever: &'static str = &name; // asks for a borrow that never ends
    println!("{forever}");
}
// ANCHOR_END: annotation
