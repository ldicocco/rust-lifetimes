// harness: compile-fail E0106
//! Case B: one named and one elided input. Naming `x` does not make it "the"
//! input: rule 2 still sees two parameters with lifetimes.

// ANCHOR: case_b
pub fn first<'a>(x: &'a str, y: &str) -> &str {
    x
}
// ANCHOR_END: case_b
