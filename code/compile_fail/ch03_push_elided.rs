// harness: compile-fail "lifetime may not live long enough"
//! Rule 1 gives every elided input its own lifetime. That is the right default
//! for most functions, but `push` needs a relation between two of them.

// ANCHOR: push
pub fn push(list: &mut Vec<&str>, item: &str) {
    list.push(item);
}
// ANCHOR_END: push
