// Three ways to give the field a meaning. Each one is a design decision.

// 1. Borrow: the lifetime becomes a parameter of the TYPE.
struct Borrowed<'a> {
    r: &'a str,
}

// 2. Own the data: no lifetime anywhere.
struct Owned {
    r: String,
}

// 3. Literals only: an explicit 'static.
struct Literal {
    r: &'static str,
}

fn wrap<'a>(s: &'a str) -> Borrowed<'a> {
    Borrowed { r: s }
}

fn main() {
    let text = String::from("hello");
    let b = wrap(&text);                 // Borrowed<'_> tied to `text`
    let o = Owned { r: text.clone() };   // independent of `text`
    let l = Literal { r: "literal" };    // fine: a literal is 'static
    println!("{} {} {}", b.r, o.r, l.r);
}
