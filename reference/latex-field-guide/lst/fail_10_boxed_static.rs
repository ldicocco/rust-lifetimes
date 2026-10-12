// trait Shape, struct Circle<'a> and its impl: exactly as in the previous listing

fn boxed(r: &f64) -> Box<dyn Shape> {
    Box::new(Circle { r })
}

fn main() {
    let r = 2.0;
    println!("{:.2}", boxed(&r).area());
}
