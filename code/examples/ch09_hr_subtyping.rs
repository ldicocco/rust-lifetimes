// harness: run
//! A higher-ranked fn pointer is a subtype of each of its instances.
//!
//! Run with: `cargo run --example ch09_hr_subtyping`

fn len(s: &str) -> usize {
    s.len()
}

fn main() {
    // ANCHOR: subtype
    // `len` works for every lifetime...
    let any: for<'a> fn(&'a str) -> usize = len;
    // ...so it can be used where only one specific lifetime is required.
    let only_static: fn(&'static str) -> usize = any;
    let local = String::from("hello");
    println!("{} {}", any(&local), only_static("hi"));
    // ANCHOR_END: subtype

    // ANCHOR: sugar
    // The elided forms are the same higher-ranked types.
    let p: fn(&str) -> &str = str::trim;
    let q: for<'a> fn(&'a str) -> &'a str = p;
    let b: Box<dyn Fn(&str) -> usize> = Box::new(|s: &str| s.len());
    let c: Box<dyn for<'a> Fn(&'a str) -> usize> = b;
    println!("{:?} {}", q("  padded  "), c("four"));
    // ANCHOR_END: sugar
}
