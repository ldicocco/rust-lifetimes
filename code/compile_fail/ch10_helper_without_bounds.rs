// harness: compile-fail E0309
//! Without the defaulted `Bounds<&'this Self>` parameter, the impl has no
//! implied `T: 'this`, and `&'this mut [T]` is not well formed.

// ANCHOR: traits
pub trait LendingIteratorLifetime<'this> {
    type Item;
}

pub struct WindowsMut<'s, T> {
    data: &'s mut [T],
    size: usize,
    start: usize,
}

impl<'this, 's, T> LendingIteratorLifetime<'this> for WindowsMut<'s, T> {
    type Item = &'this mut [T];
}
// ANCHOR_END: traits
