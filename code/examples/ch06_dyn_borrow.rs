// harness: run
//! Job 3 of `'_`: letting a trait object borrow. In a return type, `+ '_` is the
//! lifetime elision picks; in a parameter it is a fresh lifetime; in an `impl`
//! header it makes the `impl` apply to trait objects of every lifetime.
//!
//! Run with: `cargo run --example ch06_dyn_borrow`

// ANCHOR: prefixer
// The closure borrows `prefix`, and `+ '_` says so.
fn make_prefixer(prefix: &str) -> Box<dyn Fn(&str) -> String + '_> {
    Box::new(move |line| format!("{prefix}{line}"))
}
// ANCHOR_END: prefixer

// ANCHOR: run_all
// In a parameter, `+ '_` is a fresh lifetime: tasks that borrow anything at all
// are accepted, for the duration of the call.
fn run_all(tasks: Vec<Box<dyn FnMut() + '_>>) {
    for mut task in tasks {
        task();
    }
}
// ANCHOR_END: run_all

// ANCHOR: impl_dyn
trait Shape {
    fn area(&self) -> f64;
}

// `impl dyn Shape + '_` is `impl<'a> dyn Shape + 'a`: every trait object.
impl dyn Shape + '_ {
    fn describe(&self) -> String {
        format!("a shape of area {:.2}", self.area())
    }
}

fn report(shape: &dyn Shape) -> String {
    shape.describe()
}
// ANCHOR_END: impl_dyn

struct Square(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

// ANCHOR: main
fn main() {
    let prefix = String::from(">> ");
    let quote = make_prefixer(&prefix);
    println!("{}", quote("hello"));

    let mut log = Vec::new();
    let mut runs = 0;
    let tasks: Vec<Box<dyn FnMut() + '_>> =
        vec![Box::new(|| log.push("indexed")), Box::new(|| runs += 1)];
    run_all(tasks);
    println!("{log:?} after {runs} counted run(s)");

    println!("{}", report(&Square(1.5)));
}
// ANCHOR_END: main
