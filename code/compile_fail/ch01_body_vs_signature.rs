// harness: compile-fail "lifetime may not live long enough"
//! The body is checked against the signature: it may not return `y`, whose
//! lifetime is unrelated to the one promised for the result.

// ANCHOR: body
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}
// ANCHOR_END: body

pub fn caller() -> usize {
    longest("a", "bb").len()
}
