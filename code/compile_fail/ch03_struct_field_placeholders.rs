// harness: compile-fail E0261 E0106
//! Neither the anonymous lifetime nor an undeclared name helps in a field: the
//! struct must declare its lifetime parameter.

// ANCHOR: placeholders
pub struct Anonymous {
    text: &'_ str,
}

pub struct Undeclared {
    text: &'a str,
}
// ANCHOR_END: placeholders
