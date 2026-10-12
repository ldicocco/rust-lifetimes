// harness: compile-fail "lifetime may not live long enough"
//! The hidden type borrows from `x` and from `y`. `+ 'a` requires all of it to
//! outlive `'a`, which the borrow of `y` does not.

// ANCHOR: both
pub fn both<'a, 'b>(x: &'a str, y: &'b str) -> impl Iterator<Item = char> + 'a {
    x.chars().chain(y.chars())
}
// ANCHOR_END: both
