// harness: compile-fail E0521
//! With `impl FnMut(&str)`, the callback receives a borrow that is valid only
//! during the call, so it cannot store it in a vector that outlives the call.

fn for_each_line(text: &str, mut f: impl FnMut(&str)) {
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        f(line);
    }
}

// ANCHOR: keep
pub fn caller(text: &str) -> usize {
    let mut kept: Vec<&str> = Vec::new();
    for_each_line(text, |line| kept.push(line));
    kept.len()
}
// ANCHOR_END: keep
