// harness: compile-fail "lifetime may not live long enough"
//! `Iterator::Item` is one type for the whole iterator, so it cannot mention
//! the borrow of `&mut self` that each call to `next` makes.

// ANCHOR: iterator_impl
pub struct WindowsMut<'s, T> {
    data: &'s mut [T],
    size: usize,
    start: usize,
}

impl<'s, T> Iterator for WindowsMut<'s, T> {
    type Item = &'s mut [T];

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.start;
        let end = start.checked_add(self.size)?;
        let window = self.data.get_mut(start..end)?;
        self.start += 1;
        Some(window)
    }
}
// ANCHOR_END: iterator_impl
