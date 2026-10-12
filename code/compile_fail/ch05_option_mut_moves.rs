// harness: compile-fail E0382
//! Implicit reborrowing applies to a `&mut` argument, not to a `&mut` wrapped in
//! another type: an `Option<&mut u32>` is moved like any non-`Copy` value.

// ANCHOR: option
fn bump(counter: Option<&mut u32>) {
    if let Some(count) = counter {
        *count += 1;
    }
}

pub fn bump_twice(counter: Option<&mut u32>) {
    bump(counter);
    bump(counter);
}
// ANCHOR_END: option
