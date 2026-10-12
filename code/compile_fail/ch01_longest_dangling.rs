// harness: compile-fail E0597
//! The caller uses the result of `longest` after one of the inputs has been
//! dropped. Nothing is wrong in `longest`: the call contradicts its signature.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

// ANCHOR: dangling
pub fn caller() {
    let outer = String::from("a longer string");
    let result;
    {
        let inner = String::from("short");
        result = longest(outer.as_str(), inner.as_str());
    } // `inner` is dropped here...
    println!("{result}"); // ...but `result` may point into it
}
// ANCHOR_END: dangling
