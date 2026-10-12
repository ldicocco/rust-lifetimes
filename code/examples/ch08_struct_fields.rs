// harness: run
//! Trait objects in struct fields. A field of type `Box<dyn Fn(&str) -> bool>`
//! would require `'static` closures; with `+ 'a` the struct admits that its
//! rules borrow. A generic struct that declares `T: ?Sized + 'a` gives
//! `dyn Trait` the default bound `'a`.
//!
//! Run with: `cargo run --example ch08_struct_fields`

// ANCHOR: filter
/// A word filter whose rules may borrow from the code that builds it.
struct Filter<'a> {
    rules: Vec<Box<dyn Fn(&str) -> bool + 'a>>,
}

impl<'a> Filter<'a> {
    fn new() -> Self {
        Filter { rules: Vec::new() }
    }

    fn rule(mut self, rule: impl Fn(&str) -> bool + 'a) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    fn accepts(&self, word: &str) -> bool {
        self.rules.iter().all(|rule| rule(word))
    }
}
// ANCHOR_END: filter

// ANCHOR: labeled
trait Shape {
    fn area(&self) -> f64;
}

struct Circle<'a> {
    radius: &'a f64,
}

impl Shape for Circle<'_> {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

/// A labeled reference. `T: ?Sized + 'a` is declared, so in
/// `Labeled<'a, dyn Shape>` the object's default bound is `'a`.
struct Labeled<'a, T: ?Sized + 'a> {
    label: &'a str,
    value: &'a T,
}

fn describe(item: &Labeled<'_, dyn Shape>) -> String {
    format!("{} has area {:.2}", item.label, item.value.area())
}
// ANCHOR_END: labeled

fn main() {
    // ANCHOR: main_filter
    let stop_words = vec!["the".to_string(), "a".to_string()];
    let min_len = 3;
    let filter = Filter::new()
        .rule(|w| !stop_words.iter().any(|s| s == w)) // borrows `stop_words`
        .rule(|w| w.len() >= min_len); // borrows `min_len`
    let text = "the parser reads a token stream";
    let kept: Vec<&str> = text.split(' ').filter(|w| filter.accepts(w)).collect();
    println!("{kept:?}");
    // ANCHOR_END: main_filter

    // ANCHOR: main_labeled
    let radius = 1.0;
    let circle = Circle { radius: &radius };
    // The struct literal is built with `T = dyn Shape`: `&circle` is coerced.
    let item: Labeled<'_, dyn Shape> = Labeled {
        label: "unit circle",
        value: &circle,
    };
    println!("{}", describe(&item));
    // ANCHOR_END: main_labeled
}
