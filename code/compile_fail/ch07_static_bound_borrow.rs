// harness: compile-fail E0597
//! A borrow of a local variable has a lifetime shorter than `'static`, so the
//! reference type does not satisfy `T: 'static`.

fn requires_static<T: 'static>(_value: T) {}

// ANCHOR: borrow
pub fn caller() {
    let local = String::from("owned by caller");
    requires_static(&local); // `&'a String: 'static` only if `'a` is `'static`
}
// ANCHOR_END: borrow
