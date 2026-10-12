// harness: run
//! A check of the table of default object lifetime bounds. Each function is an
//! identity function between `&mut A` and `&mut B`. Since `&mut T` is invariant
//! in `T`, it compiles only if `A` and `B` are the same type: the elided form on
//! the left and the explicit form on the right.
//!
//! Run with: `cargo run --example ch08_default_table`

use std::cell::{Ref, RefCell};
use std::fmt::Display;
use std::rc::Rc;
use std::sync::Arc;

trait Shape {
    fn area(&self) -> f64;
}

struct Square(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
}

// ANCHOR: table
// No lifetime on the container: `'static`.
fn in_box<'s>(x: &'s mut Box<dyn Shape>) -> &'s mut Box<dyn Shape + 'static> {
    x
}
fn in_rc<'s>(x: &'s mut Rc<dyn Shape>) -> &'s mut Rc<dyn Shape + 'static> {
    x
}
fn in_arc<'s>(x: &'s mut Arc<dyn Shape>) -> &'s mut Arc<dyn Shape + 'static> {
    x
}

// A reference or a `Ref` to the object: the lifetime of the borrow.
fn in_ref<'s, 'a>(x: &'s mut &'a dyn Shape) -> &'s mut &'a (dyn Shape + 'a) {
    x
}
fn in_mut<'s, 'a>(x: &'s mut &'a mut dyn Shape) -> &'s mut &'a mut (dyn Shape + 'a) {
    x
}
fn in_cell_ref<'s, 'a>(
    x: &'s mut Ref<'a, dyn Shape>,
) -> &'s mut Ref<'a, dyn Shape + 'a> {
    x
}

// The innermost container decides: a `Box` behind a reference is still `'static`.
fn box_behind_ref<'s, 'a>(
    x: &'s mut &'a Box<dyn Shape>,
) -> &'s mut &'a Box<dyn Shape + 'static> {
    x
}
// ANCHOR_END: table

fn main() {
    let mut boxed: Box<dyn Shape> = Box::new(Square(1.0));
    let mut rc: Rc<dyn Shape> = Rc::new(Square(2.0));
    let mut arc: Arc<dyn Shape> = Arc::new(Square(3.0));
    let mut square = Square(4.0);
    let mut shared: &dyn Shape = &Square(5.0);
    let mut unique: &mut dyn Shape = &mut square;
    let cell: RefCell<Box<dyn Shape>> = RefCell::new(Box::new(Square(6.0)));
    let mut guard: Ref<'_, dyn Shape> = Ref::map(cell.borrow(), |b| &**b);
    let other: Box<dyn Shape> = Box::new(Square(7.0));
    let mut behind = &other;
    let areas = [
        in_box(&mut boxed).area(),
        in_rc(&mut rc).area(),
        in_arc(&mut arc).area(),
        in_ref(&mut shared).area(),
        in_mut(&mut unique).area(),
        in_cell_ref(&mut guard).area(),
        box_behind_ref(&mut behind).area(),
    ];
    println!("{areas:?}");

    // ANCHOR: body
    // In a function body an elided bound is inferred: this box may borrow `name`.
    let name = String::from("local");
    let shown: Box<dyn Display> = Box::new(&name);
    println!("{shown}");
    // ANCHOR_END: body
}
