// harness: compile-fail "lifetime may not live long enough"
//! Behind `&mut`, a `Box<dyn Shape>` cannot be viewed as a `Box<dyn Shape + 'a>`:
//! through the new view, a box holding a shorter borrow could be stored in a
//! place that promises `'static`.

pub trait Shape {
    fn area(&self) -> f64;
}

// ANCHOR: invariant
pub fn shorten_slot<'a>(slot: &'a mut Box<dyn Shape>) -> &'a mut Box<dyn Shape + 'a> {
    slot
}
// ANCHOR_END: invariant
