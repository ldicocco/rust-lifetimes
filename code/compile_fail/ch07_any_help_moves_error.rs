// harness: compile-fail E0597
//! Following the compiler's suggestion (`Box<dyn Any + 'src>`) makes `erase`
//! compile, but `Any: 'static` then implies `'src: 'static`, so the error moves
//! to every caller.

use std::any::Any;

pub struct Token<'src> {
    pub text: &'src str,
}

// ANCHOR: moved
pub fn erase<'src>(token: Token<'src>) -> Box<dyn Any + 'src> {
    Box::new(token) // compiles: the signature now implies `'src: 'static`
}

pub fn caller() {
    let source = String::from("let x = 1;");
    let erased = erase(Token { text: &source[..3] });
    println!("{}", erased.is::<Token<'static>>());
}
// ANCHOR_END: moved
