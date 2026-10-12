// harness: compile-fail E0309
//! `+ 'a` on an opaque type is a bound on the hidden type, not a capture list:
//! every type in the hidden type must outlive `'a`, including `T`.

use std::fmt::{self, Display};

pub struct Labeled<'a, T>(&'a str, T);

impl<T: Display> Display for Labeled<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

// ANCHOR: labeled
pub fn labeled<'a, T: Display>(label: &'a str, value: T) -> impl Display + 'a {
    Labeled(label, value)
}
// ANCHOR_END: labeled
