// harness: compile-fail "implementation of `Fn` is not general enough"
//! A closure whose parameter type is inferred from its first use gets one
//! specific lifetime, not a higher-ranked signature.

fn call_on_local<F: Fn(&str) -> usize>(f: F) -> usize {
    f(&String::from("local"))
}

// ANCHOR: caller
pub fn caller() -> usize {
    let count = |s| str::len(s);
    count("literal"); // fixes the parameter type: `&'static str`
    call_on_local(count)
}
// ANCHOR_END: caller
