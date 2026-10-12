// harness: compile-fail E0515
//! With the bounded signature, the slice borrows the buffer, so it cannot
//! outlive it.

use std::slice;

pub struct RawBuf {
    ptr: *mut u8,
    len: usize,
}

impl RawBuf {
    pub fn new(bytes: &[u8]) -> RawBuf {
        let boxed: Box<[u8]> = bytes.into();
        let len = boxed.len();
        let ptr = Box::into_raw(boxed).cast::<u8>();
        RawBuf { ptr, len }
    }

    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY: see `ch13_raw_buf.rs`.
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl Drop for RawBuf {
    fn drop(&mut self) {
        let slice = std::ptr::slice_from_raw_parts_mut(self.ptr, self.len);
        // SAFETY: see `ch13_raw_buf.rs`.
        drop(unsafe { Box::from_raw(slice) });
    }
}

// ANCHOR: keep_forever
pub fn keep_forever() -> &'static [u8] {
    let buf = RawBuf::new(b"temporary");
    buf.as_bytes()
}
// ANCHOR_END: keep_forever
