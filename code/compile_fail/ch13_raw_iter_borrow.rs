// harness: compile-fail E0502
//! Thanks to `PhantomData<&'a T>`, the borrow checker keeps the vector borrowed
//! while an item of the raw iterator is in use.

use std::marker::PhantomData;

pub struct RawIter<'a, T> {
    ptr: *const T,
    remaining: usize,
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
        // SAFETY: see `ch13_raw_iter.rs`.
        let item = unsafe { &*self.ptr };
        // SAFETY: see `ch13_raw_iter.rs`.
        self.ptr = unsafe { self.ptr.add(1) };
        self.remaining -= 1;
        Some(item)
    }
}

// ANCHOR: caller
pub fn caller() {
    let mut words = vec![String::from("listen"), String::from("port")];
    let first = RawIter::new(&words).next();
    words.push(String::from("8080")); // may reallocate: `first` would dangle
    println!("{first:?}");
}
// ANCHOR_END: caller
