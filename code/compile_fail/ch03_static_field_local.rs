// harness: compile-fail E0597
//! What a silent `'static` default for fields would cost: the type could only
//! hold literals and leaked data, never a borrow of a local.

// ANCHOR: literal
pub struct Literal {
    text: &'static str,
}

pub fn caller() {
    let name = String::from("hello");
    // A borrow of a local variable is not 'static:
    let lit = Literal { text: &name };
    println!("{}", lit.text);
}
// ANCHOR_END: literal
