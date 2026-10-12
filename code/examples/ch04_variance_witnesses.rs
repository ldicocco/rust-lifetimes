// harness: run
//! Witness functions: each body is just its argument, so each one compiles only if
//! the argument type is a subtype of the return type. Together they confirm the
//! covariant and contravariant entries of the variance table.
//!
//! Run with: `cargo run --example ch04_variance_witnesses`

use std::fmt::Display;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

// ANCHOR: covariant
// In every witness, 'long: 'short ("'long outlives 'short").

// `&'a T` is covariant in 'a...
fn shrink_ref<'long: 'short, 'short>(r: &'long str) -> &'short str {
    r
}

// ...and in T (here T = &'long str, behind a reference of some lifetime 'r).
fn shrink_inner<'long: 'short, 'short, 'r>(r: &'r &'long str) -> &'r &'short str {
    r
}

// Owning containers are covariant in their element type.
fn shrink_vec<'long: 'short, 'short>(v: Vec<&'long str>) -> Vec<&'short str> {
    v
}

// `&'a mut T` is covariant in 'a: the loan can be shortened...
fn shrink_loan<'long: 'short, 'short, 'v>(
    r: &'long mut &'v str,
) -> &'short mut &'v str {
    r
}
// ...but its T (here `&'v str`) must stay exactly as it is.
// ANCHOR_END: covariant

// ANCHOR: more_covariant
fn shrink_box<'long: 'short, 'short>(b: Box<&'long str>) -> Box<&'short str> {
    b
}

fn shrink_option<'long: 'short, 'short>(o: Option<&'long str>) -> Option<&'short str> {
    o
}

fn shrink_rc<'long: 'short, 'short>(r: Rc<&'long str>) -> Rc<&'short str> {
    r
}

fn shrink_arc<'long: 'short, 'short>(a: Arc<&'long str>) -> Arc<&'short str> {
    a
}

fn shrink_const_ptr<'long: 'short, 'short>(p: *const &'long str) -> *const &'short str {
    p
}

fn shrink_fn_result<'long: 'short, 'short>(
    f: fn() -> &'long str,
) -> fn() -> &'short str {
    f
}

fn shrink_dyn_bound<'long: 'short, 'short>(
    d: Box<dyn Display + 'long>,
) -> Box<dyn Display + 'short> {
    d
}

fn shrink_phantom<'long: 'short, 'short>(
    p: PhantomData<&'long str>,
) -> PhantomData<&'short str> {
    p
}
// ANCHOR_END: more_covariant

// ANCHOR: contravariant
// The direction is reversed: a callback for SHORT-lived strings is usable as a
// callback for LONG-lived ones.
fn grow_fn_arg<'long: 'short, 'short>(
    f: fn(&'short str) -> usize,
) -> fn(&'long str) -> usize {
    f
}
// ANCHOR_END: contravariant

// ANCHOR: twice
// An argument of an argument: two reversals, so covariant again.
fn shrink_twice<'long: 'short, 'short>(f: fn(fn(&'long str))) -> fn(fn(&'short str)) {
    f
}
// ANCHOR_END: twice

fn greeting() -> &'static str {
    "hello"
}

fn call_with_literal(callback: fn(&'static str)) {
    callback("a literal");
}

fn show(text: &str) {
    println!("shrink_twice:     {text}");
}

fn main() {
    let mut slot: &'static str = "slot";
    let pointer: *const &str = &"pointer";
    println!("shrink_ref:       {}", shrink_ref("literal"));
    println!("shrink_inner:     {}", shrink_inner(&"inner"));
    println!("shrink_vec:       {:?}", shrink_vec(vec!["a", "b"]));
    println!("shrink_loan:      {}", shrink_loan(&mut slot));
    println!("shrink_box:       {}", shrink_box(Box::new("boxed")));
    println!("shrink_option:    {:?}", shrink_option(Some("some")));
    println!("shrink_rc:        {}", shrink_rc(Rc::new("shared")));
    println!("shrink_arc:       {}", shrink_arc(Arc::new("atomic")));
    println!("shrink_const_ptr: {}", shrink_const_ptr(pointer).is_null());
    println!("shrink_fn_result: {}", shrink_fn_result(greeting)());
    println!("shrink_dyn_bound: {}", shrink_dyn_bound(Box::new(42)));
    let _marker: PhantomData<&str> = shrink_phantom(PhantomData);
    println!("grow_fn_arg:      {}", grow_fn_arg(str::len)("four"));
    shrink_twice(call_with_literal)(show);
}
