// harness: compile-fail E0392
//! A lifetime parameter that no field uses is an error: the compiler could not
//! tell what it means, or how the type varies with it.

// ANCHOR: raw_iter
pub struct RawIter<'a, T> {
    ptr: *const T,
    remaining: usize,
}
// ANCHOR_END: raw_iter
