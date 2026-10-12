// harness: compile-fail E0499
//! An item of a lending iterator keeps the iterator mutably borrowed, so two
//! items cannot be alive at the same time.

pub trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

// ANCHOR: two_items
pub fn first_two<L: LendingIterator>(mut lender: L) {
    let first = lender.next();
    let second = lender.next();
    drop(first);
    drop(second);
}
// ANCHOR_END: two_items
