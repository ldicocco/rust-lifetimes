// harness: compile-fail E0582 E0316 E0658
//! Things a `for<..>` binder cannot do on stable Rust 1.99.

// ANCHOR: limits
// 1. A bound lifetime used only in the output.
pub fn output_only<F>(f: F)
where
    F: for<'a> Fn() -> &'a str,
{
    let _ = f;
}

// 2. Two nested binders on the same bound.
pub fn nested<F>(f: F)
where
    for<'a> F: for<'b> Fn(&'a str, &'b str),
{
    let _ = f;
}

// 3. A binder over types.
pub fn over_types<F>(f: F)
where
    F: for<T> Fn(T),
{
    let _ = f;
}

// 4. A binder on a closure.
pub fn closure_binder() -> usize {
    let len = for<'a> |s: &'a str| -> usize { s.len() };
    len("four")
}
// ANCHOR_END: limits
