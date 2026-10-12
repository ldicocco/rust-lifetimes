// harness: compile-fail E0106
//! A fn pointer type elides like a function signature, so it fails like one.

// ANCHOR: fn_pointer
pub type Choose = fn(&str, &str) -> &str;
// ANCHOR_END: fn_pointer
