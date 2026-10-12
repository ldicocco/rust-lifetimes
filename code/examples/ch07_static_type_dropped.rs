// harness: run
//! `T: 'static` is a property of a type, not of a value: a value of a `'static`
//! type can be dropped a moment after it is created.
//!
//! Run with: `cargo run --example ch07_static_type_dropped`

// ANCHOR: noisy
/// Prints a line when it is dropped, so we can see when that happens.
struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  drop {}", self.0);
    }
}

/// Accepts any value whose type holds no borrow that can expire.
fn requires_static<T: 'static>(_value: T) {
    println!("  inside requires_static");
} // `_value` is dropped here, like any parameter
// ANCHOR_END: noisy

fn main() {
    // ANCHOR: main
    println!("call 1");
    requires_static(Noisy("first")); // `Noisy: 'static`, and it dies right away
    println!("call 2");
    let local = String::from("owned by main");
    requires_static(local); // a local `String` is fine: it is moved in
    println!("call 3");
    requires_static(7_u8);
    requires_static("a literal"); // `&'static str: 'static`
    println!("done");
    // ANCHOR_END: main
}
