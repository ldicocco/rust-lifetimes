// harness: compile-fail E0382
//! A generic parameter `T` is not known to be a reference, so the argument is
//! not reborrowed: the `&mut` is moved into the call.

// ANCHOR: generic
fn passthrough<T>(value: T) -> T {
    value
}

pub fn generic_call(words: &mut Vec<String>) {
    passthrough(words).push(String::from("zeta"));
    passthrough(words).push(String::from("eta"));
}
// ANCHOR_END: generic
