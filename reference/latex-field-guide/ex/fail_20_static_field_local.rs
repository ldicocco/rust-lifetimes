// What a silent `'static` default for fields would cost:
struct Literal {
    r: &'static str,
}

fn main() {
    let text = String::from("hello");
    let l = Literal { r: &text }; // a borrow of a local is not 'static
    println!("{}", l.r);
}
