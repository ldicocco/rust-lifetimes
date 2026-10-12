// harness: compile-fail E0521 "current limitation of the type system"
//! An adapter method hits the same limitation: `FnMut(Self::Item<'_>)` means
//! `for<'a> FnMut(Self::Item<'a>)`.

pub trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;

    // ANCHOR: for_each
    fn for_each<F>(mut self, mut f: F)
    where
        Self: Sized,
        F: FnMut(Self::Item<'_>),
    {
        while let Some(item) = self.next() {
            f(item);
        }
    }
    // ANCHOR_END: for_each
}

pub struct WindowsMut<'s, T> {
    data: &'s mut [T],
    size: usize,
    start: usize,
}

impl<'s, T> LendingIterator for WindowsMut<'s, T> {
    type Item<'a>
        = &'a mut [T]
    where
        Self: 'a;

    fn next(&mut self) -> Option<&mut [T]> {
        let start = self.start;
        let end = start.checked_add(self.size)?;
        let window = self.data.get_mut(start..end)?;
        self.start += 1;
        Some(window)
    }
}

// ANCHOR: caller
pub fn reverse_triples(values: &mut Vec<i32>) {
    let windows = WindowsMut {
        data: values,
        size: 3,
        start: 0,
    };
    windows.for_each(|triple| triple.reverse());
}
// ANCHOR_END: caller
