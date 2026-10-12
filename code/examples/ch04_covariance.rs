// harness: run
//! Covariance: a `&'static str` is accepted where a shorter `&'a str` is
//! expected, and the same holds through `Vec` and `Box`.
//!
//! Run with: `cargo run --example ch04_covariance`

// ANCHOR: pick
fn pick<'a>(preferred: &'a str, fallback: &'a str) -> &'a str {
    if preferred.is_empty() {
        fallback
    } else {
        preferred
    }
}
// ANCHOR_END: pick

fn main() {
    // ANCHOR: shrink
    let fallback: &'static str = "anonymous";
    let typed = String::from("ferris");
    // `'a` becomes (at most) the region of the borrow of `typed`; the
    // `&'static str` is used as a `&'a str`.
    println!("{}", pick(&typed, fallback));
    // ANCHOR_END: shrink

    // ANCHOR: through_types
    let defaults: Vec<&'static str> = vec!["red", "green"];
    let custom = String::from("teal");
    let mut palette: Vec<&str> = defaults; // Vec<&'static str> used as Vec<&'p str>
    palette.push(&custom);
    println!("{palette:?}");

    let boxed: Box<&'static str> = Box::new("static");
    let mut current: Box<&str> = boxed; // Box<&'static str> used as Box<&'c str>
    println!("{current}");
    current = Box::new(&custom);
    println!("{current}");
    // ANCHOR_END: through_types
}
