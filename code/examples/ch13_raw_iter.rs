// harness: run
//! `PhantomData<&'a T>` ties a raw-pointer iterator to a borrow of the slice it
//! walks: the type behaves, for the borrow checker, as if it held a `&'a T`.
//!
//! Run with: `cargo run --example ch13_raw_iter`

use std::marker::PhantomData;

// ANCHOR: raw_iter
/// Iterates over a slice with a raw pointer, like `slice::Iter` does.
pub struct RawIter<'a, T> {
    ptr: *const T,
    remaining: usize,
    /// "This type borrows `T`s for `'a`": no data, only a lifetime and a
    /// variance for the compiler.
    _borrow: PhantomData<&'a T>,
}

impl<'a, T> RawIter<'a, T> {
    pub fn new(slice: &'a [T]) -> Self {
        RawIter {
            ptr: slice.as_ptr(),
            remaining: slice.len(),
            _borrow: PhantomData,
        }
    }
}

impl<'a, T> Iterator for RawIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.remaining == 0 {
            return None;
        }
        // SAFETY: `ptr` points into the slice passed to `new`, at an element
        // not yet returned (`remaining > 0`). The slice is borrowed for `'a`,
        // so it is neither freed nor mutated while `'a` lasts, and returning
        // `&'a T` is exactly what that borrow allows.
        let item = unsafe { &*self.ptr };
        // SAFETY: there are `remaining` elements from `ptr` on, so `ptr + 1`
        // stays within the slice or one past its end.
        self.ptr = unsafe { self.ptr.add(1) };
        self.remaining -= 1;
        Some(item)
    }
}
// ANCHOR_END: raw_iter

fn main() {
    // ANCHOR: main
    let words = vec![String::from("listen"), String::from("port")];
    let longest = RawIter::new(&words).max_by_key(|w| w.len());
    println!("{longest:?}");
    // ANCHOR_END: main
}
