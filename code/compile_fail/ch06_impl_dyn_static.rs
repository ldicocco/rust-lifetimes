// harness: compile-fail E0521
//! `impl dyn Shape` is an `impl` for `dyn Shape + 'static`: its methods cannot be
//! called on a trait object that may borrow.

pub trait Shape {
    fn area(&self) -> f64;
}

// ANCHOR: impl_dyn
impl dyn Shape {
    pub fn describe(&self) -> String {
        format!("a shape of area {:.2}", self.area())
    }
}

pub fn report(shape: &dyn Shape) -> String {
    shape.describe()
}
// ANCHOR_END: impl_dyn
