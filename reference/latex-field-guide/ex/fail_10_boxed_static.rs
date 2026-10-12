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

fn boxed(r: &f64) -> Box<dyn Shape> {
    Box::new(Circle { r })
}

fn main() {
    let r = 2.0;
    println!("{:.2}", boxed(&r).area());
}
