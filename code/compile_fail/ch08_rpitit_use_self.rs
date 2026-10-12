// harness: compile-fail "must mention the `Self` type of the trait"
//! In a trait, `use<..>` must list `Self` (and the trait's other type
//! parameters): an empty list is rejected.

// ANCHOR: no_self
pub trait Inventory {
    fn ids(&self) -> impl Iterator<Item = u32> + use<>;
}
// ANCHOR_END: no_self
