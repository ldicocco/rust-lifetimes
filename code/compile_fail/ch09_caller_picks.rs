// harness: compile-fail E0597
//! A lifetime parameter of a function is chosen by the caller, so it outlives
//! the whole call: no borrow of a local variable can have it.

// ANCHOR: with_local
fn with_local<'a, F>(f: F) -> usize
where
    F: Fn(&'a str) -> usize,
{
    let local = String::from("created inside");
    f(&local)
}
// ANCHOR_END: with_local

pub fn caller() -> usize {
    with_local(|s| s.len())
}
