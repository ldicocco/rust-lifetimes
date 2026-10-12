// harness: run
//! Every type in this program satisfies `T: 'static`: none of them contains a
//! borrow that can expire.
//!
//! Run with: `cargo run --example ch07_static_table`

use std::error::Error;
use std::rc::Rc;

/// Compiles only if `T: 'static`; prints the type it was called with.
fn is_static<T: 'static>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

fn main() {
    // ANCHOR: yes
    is_static(&String::from("owned"));
    is_static(&vec![String::from("a"), String::from("b")]);
    is_static(&42_i32);
    is_static(&"a literal"); // `&'static str`
    is_static(&Rc::new(String::from("shared"))); // `Rc<T>: 'static` iff `T: 'static`
    let error: Box<dyn Error> = "boom".into(); // `Box<dyn Error + 'static>`
    is_static(&error);
    is_static(&(word_count as fn(&str) -> usize)); // a function pointer
    let owned = String::from("captured by move");
    is_static(&move || owned.len()); // a `move` closure that owns a `String`
    // ANCHOR_END: yes
}
