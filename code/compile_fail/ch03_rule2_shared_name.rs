// harness: compile-fail E0106
//! Two parameters that use the SAME lifetime `'a`. The Reference's wording ("exactly
//! one lifetime used in the parameters") would accept this; rustc counts
//! parameters, finds two, and rejects it.

// ANCHOR: shared_name
pub fn first<'a>(x: &'a str, y: &'a str) -> &str {
    x
}
// ANCHOR_END: shared_name
