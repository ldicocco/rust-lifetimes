// harness: run
//! A lending iterator: each item is a mutable window that borrows from the
//! iterator itself, so the trait needs a generic associated type.
//!
//! Run with: `cargo run --example ch10_windows_mut`

// ANCHOR: trait
pub trait LendingIterator {
    /// The item type, for each lifetime `'a` of a borrow of the iterator.
    type Item<'a>
    where
        Self: 'a;

    /// Each call borrows `self` and returns an item tied to that borrow.
    fn next(&mut self) -> Option<Self::Item<'_>>;
}
// ANCHOR_END: trait

// ANCHOR: windows_mut
/// Overlapping mutable windows of `size` elements: `[0..2]`, `[1..3]`, ...
pub struct WindowsMut<'s, T> {
    data: &'s mut [T],
    size: usize,
    start: usize,
}

impl<'s, T> WindowsMut<'s, T> {
    pub fn new(data: &'s mut [T], size: usize) -> Self {
        assert!(size > 0, "a window needs at least one element");
        WindowsMut {
            data,
            size,
            start: 0,
        }
    }
}

impl<'s, T> LendingIterator for WindowsMut<'s, T> {
    type Item<'a>
        = &'a mut [T]
    where
        Self: 'a;

    fn next(&mut self) -> Option<&mut [T]> {
        let start = self.start;
        let end = start.checked_add(self.size)?;
        // Reborrow `*self.data` for as long as `&mut self` lasts, no longer.
        let window = self.data.get_mut(start..end)?;
        self.start += 1;
        Some(window)
    }
}
// ANCHOR_END: windows_mut

fn main() {
    // ANCHOR: consume
    // Hourly readings of a counter that should never go down: repair the
    // dips by carrying the previous value forward.
    let mut readings = vec![3, 5, 4, 8, 7, 9];
    let mut windows = WindowsMut::new(&mut readings, 2);
    while let Some(pair) = windows.next() {
        if pair[1] < pair[0] {
            pair[1] = pair[0];
        }
    }
    println!("{readings:?}");
    // ANCHOR_END: consume
    assert_eq!(readings, [3, 5, 5, 8, 8, 9]);
}
