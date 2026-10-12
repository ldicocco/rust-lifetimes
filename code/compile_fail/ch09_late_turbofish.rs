// harness: compile-fail E0794
//! Late-bound lifetimes are chosen at each call, so they cannot be specified
//! with a turbofish.

// ANCHOR: turbofish
fn late<'a>(s: &'a str) -> &'a str {
    s
}

pub fn caller() -> &'static str {
    let f = late::<'static>;
    f("literal")
}
// ANCHOR_END: turbofish
