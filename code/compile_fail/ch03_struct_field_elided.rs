// harness: compile-fail E0106
//! The simplest possible case: one struct, one field, one reference.

// ANCHOR: holder
pub struct Holder {
    text: &str,
}
// ANCHOR_END: holder
