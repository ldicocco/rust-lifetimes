// harness: compile-fail E0597
//! `&mut T` is invariant in `T`. If it were covariant, `overwrite` could store a
//! borrow of `local` in a variable that promises a `&'static str`.

// ANCHOR: overwrite
fn overwrite<'a>(slot: &mut &'a str, value: &'a str) {
    *slot = value;
}

pub fn caller() {
    let mut name: &'static str = "static";
    {
        let local = String::from("local");
        overwrite(&mut name, &local); // would leave `name` dangling
    }
    println!("{name}");
}
// ANCHOR_END: overwrite
