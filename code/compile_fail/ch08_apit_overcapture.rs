// harness: compile-fail E0597
//! The returned `impl Display` captures the anonymous type parameter and the
//! lifetime of the reference; `use<..>` cannot fix it while the parameter has no
//! name.

use std::fmt::Display;

// ANCHOR: shout
pub fn shout(value: &impl Display) -> impl Display {
    format!("{value}!")
}

pub fn caller() -> String {
    let loud;
    {
        let word = String::from("hello");
        loud = shout(&word);
    }
    loud.to_string()
}
// ANCHOR_END: shout
