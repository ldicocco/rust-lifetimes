// harness: compile-fail E0106
//! `Fn` sugar follows the elision rules of functions: with two reference
//! inputs, an elided output lifetime has no source.

// ANCHOR: bound
pub fn split_head<F>(f: F) -> String
where
    F: Fn(&str, &str) -> &str,
{
    f("key=value", "=").to_string()
}
// ANCHOR_END: bound
