// F must work for *every* lifetime 'a, not one fixed lifetime.
fn apply<F>(f: F, s: &str) -> usize
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    f(s).len()
}

// A bound that genuinely needs `for<'a>`: `&C` must be iterable for any 'a.
fn sum_all<C>(c: &C) -> i32
where
    for<'a> &'a C: IntoIterator<Item = &'a i32>,
{
    c.into_iter().sum()
}

fn main() {
    println!("{}", apply(|s| s.trim(), "  hi  "));
    println!("{}", sum_all(&vec![1, 2, 3]));
}
