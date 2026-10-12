// Case A: two elided inputs, elided output
fn a(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

// Case B: one named + one elided input, elided output
fn b<'a>(x: &'a str, y: &str) -> &str {
    x
}

fn main() {}
