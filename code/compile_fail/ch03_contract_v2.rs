// harness: compile-fail E0597
//! Version 2: only the body changed (it returns `y` now). If contracts were
//! inferred from bodies, this would be the new signature, and the unchanged
//! caller would break.

// ANCHOR: v2
pub fn pick<'a>(_x: &str, y: &'a str) -> &'a str {
    y
}
// ANCHOR_END: v2

// ANCHOR: caller
pub fn caller() {
    let long_lived = String::from("long-lived");
    let result;
    {
        let short_lived = String::from("short");
        // Identical caller code:
        result = pick(&long_lived, &short_lived);
    }
    println!("{result}");
}
// ANCHOR_END: caller
