// harness: compile-fail E0106
//! Case A: two elided inputs and an elided output. Rule 1 gives the inputs two
//! lifetimes; rules 2 and 3 do not apply, so nothing names the output's lifetime.

// ANCHOR: case_a
pub fn longer(x: &str, y: &str) -> &str {
    if x.len() >= y.len() { x } else { y }
}
// ANCHOR_END: case_a
