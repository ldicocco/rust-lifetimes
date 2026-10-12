// harness: run
//! Five ways to get a `&'static` reference: literals, `static` items, `const`
//! items, constant promotion and leaking.
//!
//! Run with: `cargo run --example ch07_static_sources`

// ANCHOR: items
// The type of a string literal is `&'static str`: the bytes are part of the binary.
static BANNER: &str = "lifetimes"; // elided: `&'static str`
const LIMITS: &[u32] = &[1, 2, 3]; // elided: `&'static [u32]`

// A `static` item lives at one address for the whole run, so `&GREETING` is
// `&'static String`, even though `String` has a destructor (it never runs).
static GREETING: String = String::new();
// ANCHOR_END: items

// ANCHOR: promotion
fn promoted_number() -> &'static i32 {
    &42 // a constant expression: promoted to an anonymous static
}

fn promoted_table() -> &'static [&'static str] {
    &["fn", "let", "match"] // an array of literals is a constant too
}
// ANCHOR_END: promotion

// ANCHOR: leak
fn leaked(name: String) -> &'static str {
    // The heap buffer is never freed: the reference stays valid until exit.
    name.leak()
}
// ANCHOR_END: leak

fn main() {
    // ANCHOR: main
    let literal: &'static str = "a literal";
    let empty: &'static String = &GREETING;
    let label: &'static str = leaked(format!("{}-{}", "worker", 7));
    println!("literal:  {literal}");
    println!("static:   {BANNER} and {empty:?}");
    println!("const:    {LIMITS:?}");
    println!("promoted: {} and {:?}", promoted_number(), promoted_table());
    println!("leaked:   {label}");
    // ANCHOR_END: main
}
