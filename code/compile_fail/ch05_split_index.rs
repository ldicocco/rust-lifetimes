// harness: compile-fail E0499
//! The borrow checker does not compare index values: to it, `words[0]` and
//! `words[1]` are both `words[_]`, which may be the same element.

// ANCHOR: index
pub fn swap_first_two(words: &mut [String]) {
    let a = &mut words[0];
    let b = &mut words[1];
    std::mem::swap(a, b);
}
// ANCHOR_END: index
