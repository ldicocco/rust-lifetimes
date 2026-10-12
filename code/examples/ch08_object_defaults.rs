// harness: run
//! Default object lifetime bounds in signatures: `Box<dyn Shape>` means
//! `Box<dyn Shape + 'static>`, while `&'a dyn Shape` means `&'a (dyn Shape + 'a)`.
//! Where a box must hold a value that borrows, the bound is written.
//!
//! Run with: `cargo run --example ch08_object_defaults`

// ANCHOR: shape
trait Shape {
    fn area(&self) -> f64;
}

/// A circle that borrows its radius, from a configuration for example.
struct Circle<'a> {
    radius: &'a f64,
}

impl Shape for Circle<'_> {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

/// A square that owns its data.
struct Square(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}
// ANCHOR_END: shape

// ANCHOR: boxed
// The default would be `Box<dyn Shape + 'static>`: the bound must be written.
fn boxed_circle<'a>(radius: &'a f64) -> Box<dyn Shape + 'a> {
    Box::new(Circle { radius })
}

// A square borrows nothing, so the `'static` default is the right contract.
fn boxed_square(side: f64) -> Box<dyn Shape> {
    Box::new(Square(side))
}
// ANCHOR_END: boxed

// ANCHOR: refs
// The defaults are already right here: `&'x [&'y (dyn Shape + 'y)]`.
fn total(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|shape| shape.area()).sum()
}
// ANCHOR_END: refs

// ANCHOR: main
fn main() {
    let radius = 2.0;
    let circle = boxed_circle(&radius);
    let square = boxed_square(3.0);
    let local = Circle { radius: &radius };
    println!("{:.2} {:.2}", circle.area(), square.area());
    println!("{:.2}", total(&[&local, &*circle, &*square]));
}
// ANCHOR_END: main
