// harness: compile-fail E0597
//! With a caller-chosen `'x`, the borrows of the owned `c` would have to last
//! for the whole call, but `c` is dropped at the end of the body.

// ANCHOR: stats
pub fn stats<'x, C: 'x>(c: C) -> (i32, usize)
where
    &'x C: IntoIterator<Item = &'x i32>,
{
    let sum = (&c).into_iter().sum();
    let count = (&c).into_iter().count();
    (sum, count)
}
// ANCHOR_END: stats
