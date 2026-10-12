// harness: compile-fail E0597
//! The body never returns `y`, but the signature says that it may, and callers
//! are checked against the signature only.

// ANCHOR: first
fn first<'a>(x: &'a str, y: &'a str) -> &'a str {
    println!("ignoring {y:?}");
    x
}
// ANCHOR_END: first

// ANCHOR: caller
pub fn caller() {
    let outer = String::from("a longer string");
    let result;
    {
        let inner = String::from("short");
        result = first(outer.as_str(), inner.as_str());
    }
    println!("{result}");
}
// ANCHOR_END: caller
