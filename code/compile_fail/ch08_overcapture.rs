// harness: compile-fail E0597
//! Edition 2024 captures every lifetime in scope, including the one of `prefix`,
//! which the iterator never uses: callers must keep `prefix` alive as long as
//! the iterator.

// ANCHOR: after_prefix
pub fn after_prefix<'s>(s: &'s str, prefix: &str) -> impl Iterator<Item = char> {
    s.strip_prefix(prefix).unwrap_or(s).chars()
}
// ANCHOR_END: after_prefix

// ANCHOR: caller
pub fn caller() -> String {
    let text = String::from("> hello");
    let chars;
    {
        let prefix = String::from("> ");
        chars = after_prefix(&text, &prefix);
    } // `prefix` is dropped here
    chars.collect()
}
// ANCHOR_END: caller
