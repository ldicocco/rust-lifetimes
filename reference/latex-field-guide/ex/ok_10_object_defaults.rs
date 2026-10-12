trait Shape {
    fn area(&self) -> f64;
}

struct Circle<'a> {
    r: &'a f64,
}

impl Shape for Circle<'_> {
    fn area(&self) -> f64 {
        3.14159 * self.r * self.r
    }
}

// Must say `+ 'a`: the default for Box<dyn Shape> would be 'static.
fn boxed<'a>(r: &'a f64) -> Box<dyn Shape + 'a> {
    Box::new(Circle { r })
}

// Here the default is already right: &'x [&'y (dyn Shape + 'y)]
fn total(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn main() {
    let r = 2.0;
    let b = boxed(&r);
    let c = Circle { r: &r };
    println!("{:.2} {:.2}", b.area(), total(&[&c, &*b]));
}
