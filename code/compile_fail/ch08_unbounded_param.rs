// harness: compile-fail E0597
//! Without a declared `T: 'a`, the container gives no bound, and
//! `Labeled<'_, dyn Shape>` means `Labeled<'_, dyn Shape + 'static>`, even though
//! the field is a `&'a T`.

pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle<'a> {
    pub radius: &'a f64,
}

impl Shape for Circle<'_> {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// ANCHOR: unbounded
pub struct Labeled<'a, T: ?Sized> {
    pub label: &'a str,
    pub value: &'a T,
}

pub fn describe(item: &Labeled<'_, dyn Shape>) -> String {
    format!("{} has area {:.2}", item.label, item.value.area())
}

pub fn caller() -> String {
    let radius = 1.0;
    let circle = Circle { radius: &radius };
    describe(&Labeled {
        label: "unit circle",
        value: &circle,
    })
}
// ANCHOR_END: unbounded
