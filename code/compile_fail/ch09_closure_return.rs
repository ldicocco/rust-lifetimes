// harness: compile-fail "lifetime may not live long enough"
//! A closure stored in a variable cannot return a reference tied to its
//! argument: its return type is inferred as one fixed lifetime. Annotating the
//! return type does not change that.

// ANCHOR: closures
pub fn caller() {
    let first_word = |s: &str| s.split(' ').next().unwrap_or("");
    let annotated = |s: &str| -> &str { s.split(' ').next().unwrap_or("") };
    let text = String::from("hello world");
    println!("{} {}", first_word(&text), annotated(&text));
}
// ANCHOR_END: closures
