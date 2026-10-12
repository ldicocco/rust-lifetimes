// harness: run
//! Three ways to get a closure (or function) whose returned reference is tied to
//! its argument.
//!
//! Run with: `cargo run --example ch09_closure_fixes`

// ANCHOR: helpers
/// Identity function whose only job is to give the closure an expected
/// higher-ranked signature.
fn hr<F: Fn(&str) -> &str>(f: F) -> F {
    f
}

fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap_or("")
}

fn apply<F: Fn(&str) -> &str>(f: F, s: &str) -> usize {
    f(s).len()
}
// ANCHOR_END: helpers

fn main() {
    // ANCHOR: fixes
    let text = String::from("hello world");
    // 1. Pass the closure directly to a function with a higher-ranked bound.
    let n = apply(|s| s.trim(), &text);
    // 2. Route a stored closure through an identity function with that bound.
    let a = hr(|s| s.split(' ').next().unwrap_or(""));
    // 3. Use a fn item: it is generic over the (late-bound) lifetime.
    let b = first_word;
    println!("{n} {} {}", a(&text), b(&text));
    // ANCHOR_END: fixes
}
