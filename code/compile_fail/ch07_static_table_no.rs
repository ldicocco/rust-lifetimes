// harness: compile-fail E0597 E0373
//! Three types that do not satisfy `T: 'static`: each holds a borrow of the
//! local variable `text`.

/// A view into some text: a struct with a lifetime parameter.
pub struct Line<'a>(&'a str);

fn is_static<T: 'static>(_: &T) {}

// ANCHOR: no
pub fn caller() {
    let text = String::from("local text");
    is_static(&text.as_str()); // `&'a str` with a local `'a`
    is_static(&Line(&text)); // `Line<'a>` with a local `'a`
    is_static(&|| text.len()); // a closure that borrows `text`
}
// ANCHOR_END: no
