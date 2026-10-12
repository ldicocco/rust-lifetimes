// harness: compile-fail "missing required bound on `Item`"
//! A GAT used together with `&mut self` must declare `where Self: 'a`.

// ANCHOR: trait
pub trait LendingIterator {
    type Item<'a>;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}
// ANCHOR_END: trait
