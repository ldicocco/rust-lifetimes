// harness: run
//! The `PhantomData` marker decides how a type varies with its lifetime
//! parameter. A covariant handle can be shrunk; a contravariant one can be
//! grown.
//!
//! Run with: `cargo run --example ch13_variance_markers`

use std::marker::PhantomData;

// ANCHOR: markers
/// Covariant in `'a`, like `&'a T`.
pub struct Shared<'a> {
    _marker: PhantomData<&'a ()>,
}

/// Contravariant in `'a`, like the argument of `fn(&'a ())`.
pub struct Sink<'a> {
    _marker: PhantomData<fn(&'a ())>,
}

/// Invariant in `'a`: the argument and the result of a function at once.
pub struct Exact<'a> {
    _marker: PhantomData<fn(&'a ()) -> &'a ()>,
}
// ANCHOR_END: markers

// ANCHOR: conversions
/// Covariance: a handle for a longer region can be used where a shorter one
/// is expected.
fn shrink<'long: 'short, 'short>(handle: Shared<'long>) -> Shared<'short> {
    handle
}

/// Contravariance: the other way around.
fn grow<'long: 'short, 'short>(sink: Sink<'short>) -> Sink<'long> {
    sink
}

/// Invariance: only the same region.
fn same<'a>(exact: Exact<'a>) -> Exact<'a> {
    exact
}
// ANCHOR_END: conversions

fn main() {
    let shared: Shared<'static> = Shared {
        _marker: PhantomData,
    };
    let _shorter = shrink(shared);
    let sink = Sink {
        _marker: PhantomData,
    };
    let _longer: Sink<'static> = grow(sink);
    let _exact = same(Exact {
        _marker: PhantomData,
    });
    println!("covariant, contravariant and invariant handles all compile");
}
