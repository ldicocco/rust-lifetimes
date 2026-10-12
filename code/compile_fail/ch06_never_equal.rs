// harness: compile-fail "lifetime may not live long enough"
//! Every `'_` is a different lifetime: writing it twice does not tie two
//! positions together.

// ANCHOR: remember
pub fn remember(seen: &mut Vec<&'_ str>, word: &'_ str) {
    seen.push(word);
}
// ANCHOR_END: remember
