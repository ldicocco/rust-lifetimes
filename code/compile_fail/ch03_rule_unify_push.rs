// harness: compile-fail E0502
//! Rule B applied to `push`: unifying every lifetime puts the vector's own
//! lifetime on the `&mut` borrow, and the vector stays borrowed for good.

// ANCHOR: rule_b_push
pub fn push<'a>(list: &'a mut Vec<&'a str>, item: &'a str) {
    list.push(item);
}
// ANCHOR_END: rule_b_push

// ANCHOR: caller
pub fn caller() {
    let first = String::from("first");
    let mut list: Vec<&str> = Vec::new();
    push(&mut list, &first);
    println!("{} items", list.len()); // any later use of `list` is rejected
}
// ANCHOR_END: caller
