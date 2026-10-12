// harness: compile-fail "lifetime may not live long enough"
//! An invariant marker forbids shrinking the lifetime, which a covariant one
//! allows.

use std::marker::PhantomData;

pub struct Exact<'a> {
    _marker: PhantomData<fn(&'a ()) -> &'a ()>,
}

// ANCHOR: shrink
pub fn shrink<'long: 'short, 'short>(exact: Exact<'long>) -> Exact<'short> {
    exact
}
// ANCHOR_END: shrink
