// harness: compile-fail "lifetime may not live long enough"
//! A boxed trait object in a signature has a hidden `+ 'static`: the closure
//! may not borrow `prefix`.

// ANCHOR: prefixer
pub fn make_prefixer(prefix: &str) -> Box<dyn Fn(&str) -> String> {
    Box::new(move |line| format!("{prefix}{line}"))
}
// ANCHOR_END: prefixer
