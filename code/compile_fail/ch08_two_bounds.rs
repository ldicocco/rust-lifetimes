// harness: compile-fail E0228
//! A container whose parameter has two lifetime bounds gives no unique default:
//! the bound must be written.

pub trait Shape {
    fn area(&self) -> f64;
}

// ANCHOR: two
/// Pairs a shape with the labels of two different documents.
pub struct Labeled<'a, 'b, T: ?Sized + 'a + 'b> {
    pub title: &'a str,
    pub note: &'b str,
    pub shape: Box<T>,
}

pub fn area_of(item: &Labeled<'_, '_, dyn Shape>) -> f64 {
    item.shape.area()
}
// ANCHOR_END: two
