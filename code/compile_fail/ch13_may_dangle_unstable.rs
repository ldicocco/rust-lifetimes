// harness: compile-fail E0658
//! `#[may_dangle]` is unstable (feature `dropck_eyepatch`).

pub struct Log<T> {
    entries: Vec<T>,
}

// ANCHOR: may_dangle
// SAFETY: `drop` never reads or writes a `T`; it only reads the length.
unsafe impl<#[may_dangle] T> Drop for Log<T> {
    fn drop(&mut self) {
        println!("dropping a log of {} entries", self.entries.len());
    }
}
// ANCHOR_END: may_dangle
