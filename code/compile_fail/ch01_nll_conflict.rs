// harness: compile-fail E0502
//! Move the use of `first` after the `push` and the shared borrow is live
//! across the mutable one.

// ANCHOR: conflict
pub fn caller() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4); // needs `&mut v` while `first` is still live
    println!("{first}"); // this later use keeps the shared borrow alive
}
// ANCHOR_END: conflict
