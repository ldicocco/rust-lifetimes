// harness: compile-fail "implementation of `Fn` is not general enough"
//! A redundant `T: 'a` bound makes `'a` early-bound: the function item is then
//! no longer generic over `'a` at each call, and does not satisfy a
//! higher-ranked bound.

// ANCHOR: functions
fn apply<F: Fn(&[i32]) -> &i32>(f: F) -> i32 {
    let v = vec![3, 1, 2];
    *f(&v)
}

fn first_late<T>(v: &[T]) -> &T {
    &v[0]
}

fn first_early<'a, T: 'a>(v: &'a [T]) -> &'a T {
    &v[0]
}

pub fn caller() -> i32 {
    apply(first_late) + apply(first_early)
}
// ANCHOR_END: functions
