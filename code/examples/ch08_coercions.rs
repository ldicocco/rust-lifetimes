// harness: run
//! Unsizing coercions turn a pointer to a concrete type into a pointer to a trait
//! object. The object's lifetime bound is checked at that moment, and a trait
//! object with a long bound can be used where a shorter one is expected.
//!
//! Run with: `cargo run --example ch08_coercions`

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

struct Square(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

// ANCHOR: largest
fn largest<'a>(shapes: &[&'a dyn Shape]) -> &'a dyn Shape {
    let mut best = shapes[0];
    for &shape in shapes {
        if shape.area() > best.area() {
            best = shape;
        }
    }
    best
}
// ANCHOR_END: largest

// ANCHOR: shorten
// A `'static` object is accepted where a shorter bound is expected...
fn shorten<'a>(shape: Box<dyn Shape>) -> Box<dyn Shape + 'a> {
    shape
}

// ...even behind `&mut`: the coercion builds a new pointer to the same object.
fn shorten_mut<'a>(shape: &'a mut (dyn Shape + 'static)) -> &'a mut (dyn Shape + 'a) {
    shape
}
// ANCHOR_END: shorten

fn main() {
    // ANCHOR: coerce
    let (r1, r2) = (1.0, 2.0);
    let small = Circle { radius: &r1 };
    let big = Circle { radius: &r2 };
    // `&Circle<'_>` to `&dyn Shape`: the array's type asks for the coercion.
    let shapes: [&dyn Shape; 2] = [&small, &big];
    println!("largest: {:.2}", largest(&shapes).area());

    // `Box<Circle<'_>>` to `Box<dyn Shape + '_>`: the bound is inferred here.
    let boxed: Box<dyn Shape + '_> = Box::new(Circle { radius: &r2 });
    println!("boxed: {:.2}", boxed.area());
    // ANCHOR_END: coerce

    let mut square = Square(3.0);
    println!("{:.2}", shorten(Box::new(Square(1.5))).area());
    println!("{:.2}", shorten_mut(&mut square).area());
}
