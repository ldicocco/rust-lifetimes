// harness: compile-fail E0502
//! An `impl Trait` in a trait captures the lifetime of `&self`, in every edition:
//! callers see the trait's signature, not what an implementation returns.

// ANCHOR: captures
pub trait Inventory {
    fn ids(&self) -> impl Iterator<Item = u32>;
}

pub fn restock<I: Inventory>(inventory: &mut I, add: impl FnOnce(&mut I)) -> u32 {
    let before = inventory.ids();
    add(inventory);
    before.sum()
}
// ANCHOR_END: captures
