// harness: compile-fail E0637
//! `'_` is not allowed in a `where` clause or in the bounds of a generic
//! parameter: there is no elision rule for it to follow there.

// ANCHOR: bound
pub fn longest_line<I>(lines: I) -> usize
where
    I: Iterator<Item = &'_ str>,
{
    lines.map(str::len).max().unwrap_or(0)
}
// ANCHOR_END: bound
