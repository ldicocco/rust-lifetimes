// harness: compile-fail E0700
//! Applying rustc's suggestion `use<>` literally: the hidden type borrows `s`,
//! so it must capture `'s`.

// ANCHOR: nothing
fn after_prefix<'s>(s: &'s str, prefix: &str) -> impl Iterator<Item = char> + use<> {
    s.strip_prefix(prefix).unwrap_or(s).chars()
}
// ANCHOR_END: nothing
