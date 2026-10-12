// harness: compile-fail E0502
//! While a mutable borrow is live, the value cannot even be read through its
//! owner.

// ANCHOR: mutable_then_shared
pub fn caller() {
    let mut log = String::from("start");
    let writer = &mut log;
    println!("{log}"); // reading `log` needs a shared borrow
    writer.push_str(" end");
}
// ANCHOR_END: mutable_then_shared
