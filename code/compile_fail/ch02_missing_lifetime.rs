// harness: compile-fail E0106
//! A struct field that is a reference needs a lifetime, and the struct must
//! declare it as a parameter.

// ANCHOR: missing
pub struct Parser {
    input: &str,
    pos: usize,
}
// ANCHOR_END: missing
