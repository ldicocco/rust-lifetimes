fn pick(a: &str, b: &str) -> &str { a }
fn pick_underscore(a: &str, b: &str) -> &'_ str { a }

fn main() {
    println!("{} {}", pick("x", "y"), pick_underscore("x", "y"));
}
