// harness: run
//! Raw pointers carry no lifetime. Turning one into a reference produces a
//! lifetime that only the signature can bound: `as_bytes` ties it to `&self`,
//! `as_bytes_unbounded` lets the caller pick any lifetime at all.
//!
//! Run with: `cargo run --example ch13_raw_buf`

use std::slice;

// ANCHOR: raw_buf
/// An owned byte buffer kept as a raw pointer and a length, as you might get
/// from a C library or a custom allocator.
pub struct RawBuf {
    ptr: *mut u8,
    len: usize,
}

impl RawBuf {
    pub fn new(bytes: &[u8]) -> RawBuf {
        let boxed: Box<[u8]> = bytes.into();
        let len = boxed.len();
        // `into_raw` gives up ownership; `Drop` below takes it back.
        let ptr = Box::into_raw(boxed).cast::<u8>();
        RawBuf { ptr, len }
    }
}

impl Drop for RawBuf {
    fn drop(&mut self) {
        let slice = std::ptr::slice_from_raw_parts_mut(self.ptr, self.len);
        // SAFETY: `ptr` and `len` came from `Box::into_raw` in `new`, and this
        // is the only place that turns them back into a box, exactly once.
        drop(unsafe { Box::from_raw(slice) });
    }
}
// ANCHOR_END: raw_buf

// ANCHOR: bounded
impl RawBuf {
    /// The elided signature reads `fn as_bytes<'a>(&'a self) -> &'a [u8]`.
    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY: `ptr` points to `len` initialized bytes owned by `self`.
        // They stay allocated while `self` is alive, and no method mutates
        // them, so they are valid and unchanged for the lifetime of `&self`,
        // which is the lifetime the signature gives the slice.
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}
// ANCHOR_END: bounded

// ANCHOR: unbounded
impl RawBuf {
    /// UNSOUND: the caller picks `'a`, and nothing ties it to `self`.
    #[allow(dead_code)] // only used by `keep_forever`, which is never called
    pub fn as_bytes_unbounded<'a>(&self) -> &'a [u8] {
        // SAFETY: (incomplete) the bytes are valid while `self` is alive, but
        // this signature lets the caller keep the slice longer than that.
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

/// Compiles, and returns a dangling slice: `buf` is freed at the end of the
/// function. Never called: calling it is undefined behavior.
#[allow(dead_code)]
fn keep_forever() -> &'static [u8] {
    let buf = RawBuf::new(b"temporary");
    buf.as_bytes_unbounded()
}
// ANCHOR_END: unbounded

fn main() {
    // ANCHOR: no_lifetime
    let pointer: *const String = {
        let temporary = String::from("temporary");
        &temporary // a `&String` coerced to a raw pointer: the borrow ends here
    }; // `temporary` is dropped; `pointer` dangles, and that is allowed
    assert!(!pointer.is_null()); // safe: this does not dereference it
    // ANCHOR_END: no_lifetime

    // ANCHOR: main
    let buf = RawBuf::new(b"listen port 8080");
    let bytes = buf.as_bytes(); // borrows `buf`
    let first_word = bytes.split(|&b| b == b' ').next().unwrap_or(&[]);
    println!("{}", String::from_utf8_lossy(first_word));
    drop(buf); // fine: `bytes` and `first_word` are no longer used
    // ANCHOR_END: main
}
