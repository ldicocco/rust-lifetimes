// harness: compile-fail E0499
//! Two mutable borrows of the same value may not be live at the same point.

// ANCHOR: two_mutable
pub fn caller() {
    let mut log = String::new();
    let header = &mut log;
    let body = &mut log; // `header` is still live: it is used below
    header.push_str("== report ==\n");
    body.push_str("all good\n");
}
// ANCHOR_END: two_mutable
