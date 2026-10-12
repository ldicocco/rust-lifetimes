// harness: compile-fail E0277
//! A `for` loop needs `IntoIterator`, which a lending iterator does not
//! implement.

pub trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

// ANCHOR: for_loop
pub fn count_items<L: LendingIterator>(lender: L) -> usize {
    let mut count = 0;
    for _item in lender {
        count += 1;
    }
    count
}
// ANCHOR_END: for_loop
