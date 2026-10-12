// harness: compile-fail E0227
//! A trait with two lifetime bounds, neither of them `'static`, gives no default:
//! the bound of the trait object must be written.

// ANCHOR: two_trait
/// A table whose keys live for `'k` and whose values live for `'v`.
pub trait Lookup<'k, 'v>: 'k + 'v {
    fn get(&self, key: &str) -> Option<&'v str>;
}

pub type BoxedLookup<'k, 'v> = Box<dyn Lookup<'k, 'v>>;
// ANCHOR_END: two_trait
