// harness: run
//! A struct field cannot elide its lifetime. Three ways to give it a meaning,
//! each one a design decision.
//!
//! Run with: `cargo run --example ch03_struct_field_choices`

// ANCHOR: choices
// 1. Borrow: the lifetime becomes a parameter of the TYPE.
struct Borrowed<'a> {
    text: &'a str,
}

// 2. Own the data: no lifetime anywhere.
struct Owned {
    text: String,
}

// 3. Literals (or leaked data) only: an explicit `'static`.
struct Literal {
    text: &'static str,
}
// ANCHOR_END: choices

// ANCHOR: uses
// Once the parameter is declared, signatures that USE the type elide as usual:
// the compiler reads fn wrap<'a>(text: &'a str) -> Borrowed<'a>
fn wrap(text: &str) -> Borrowed<'_> {
    Borrowed { text }
}
// ANCHOR_END: uses

fn main() {
    let input = String::from("hello");
    let borrowed = wrap(&input); // tied to `input`
    let copy = input.clone();
    let owned = Owned { text: copy }; // independent of `input`
    let literal = Literal { text: "literal" }; // fine: a literal is 'static
    println!("{} {} {}", borrowed.text, owned.text, literal.text);
}
