// harness: compile-fail E0506
//! Assigning to a value is a mutation too: it cannot happen while the old
//! value is borrowed.

// ANCHOR: assign
pub fn caller() {
    let mut title = String::from("draft");
    let shown = &title;
    title = String::from("final"); // would drop the string `shown` points to
    println!("{shown}");
}
// ANCHOR_END: assign
