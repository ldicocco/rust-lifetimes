// harness: compile-fail E0597
//! Function arguments are contravariant: a callback that accepts only `'static`
//! strings cannot stand in for one that must accept a short-lived string.

// ANCHOR: callback
fn apply<'a>(callback: fn(&'a str) -> usize, text: &'a str) -> usize {
    callback(text)
}

fn only_static(text: &'static str) -> usize {
    text.len()
}

pub fn caller() -> usize {
    let local = String::from("local");
    apply(only_static, &local)
}
// ANCHOR_END: callback
