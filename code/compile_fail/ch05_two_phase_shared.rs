// harness: compile-fail E0502
//! A two-phase borrow tolerates reads *before* it is activated, not a shared
//! borrow that is still in use when the call starts.

// ANCHOR: shared
pub fn double(mut text: String) -> String {
    text.push_str(&text);
    text
}
// ANCHOR_END: shared
