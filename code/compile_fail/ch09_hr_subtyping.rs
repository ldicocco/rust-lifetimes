// harness: compile-fail E0308
//! The reverse direction: a function that accepts only `&'static str` is not a
//! function that accepts every `&str`.

// ANCHOR: reverse
fn static_only(s: &'static str) -> usize {
    s.len()
}

pub fn caller() {
    let f: fn(&'static str) -> usize = static_only;
    let any: for<'a> fn(&'a str) -> usize = f;
    println!("{}", any("hi"));
}
// ANCHOR_END: reverse
