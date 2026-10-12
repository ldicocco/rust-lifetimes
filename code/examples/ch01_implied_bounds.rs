// harness: run
//! Implied bounds: a type such as `&'a &'b str` or `&'a T` is only well formed
//! if what it points to outlives `'a`, and the compiler assumes it.
//!
//! Run with: `cargo run --example ch01_implied_bounds`

// ANCHOR: implied
// `&'a &'b str` is a valid type only if `'b: 'a`, so the body may rely on it.
fn inner<'a, 'b>(outer: &'a &'b str) -> &'a str {
    *outer // a `&'b str`, returned as a `&'a str`
}

// No parameter type implies `'b: 'a` here, so the signature must say it.
fn pick<'a, 'b>(x: &'a str, y: &'b str, first: bool) -> &'a str
where
    'b: 'a,
{
    if first { x } else { y }
}

// A field `&'a T` implies `T: 'a`: the struct need not write it.
struct Labeled<'a, T> {
    label: &'a str,
    value: &'a T,
}

fn value_of<'a, T>(labeled: &Labeled<'a, T>) -> &'a T {
    labeled.value
}
// ANCHOR_END: implied

fn main() {
    let name = String::from("config.toml");
    let borrowed: &str = &name;
    println!("{}", inner(&borrowed));
    println!("{}", pick("first", &name, false));
    let port = 8080;
    let labeled = Labeled {
        label: "port",
        value: &port,
    };
    println!("{} = {}", labeled.label, value_of(&labeled));
}
