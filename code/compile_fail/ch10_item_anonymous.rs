// harness: compile-fail E0637
//! `'_` cannot stand for "the borrow of `self` in `next`": an associated type
//! has no such lifetime in scope.

pub struct WindowsMut<'s, T> {
    data: &'s mut [T],
    size: usize,
    start: usize,
}

// ANCHOR: impl
impl<'s, T> Iterator for WindowsMut<'s, T> {
    type Item = &'_ mut [T];
    // ANCHOR_END: impl

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
