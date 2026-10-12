// harness: compile-fail E0502
//! Two-phase borrows apply to the implicit `&mut` of a method call, not to a
//! `&mut` you write yourself: the same call in function syntax is rejected.

// ANCHOR: explicit
pub fn record_length(mut lengths: Vec<usize>) -> Vec<usize> {
    Vec::push(&mut lengths, lengths.len());
    lengths
}
// ANCHOR_END: explicit
