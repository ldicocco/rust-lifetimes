// Edition 2021: `impl Trait` does not capture the elided lifetime of `s`.
fn chars_of(s: &str) -> impl Iterator<Item = char> {
    s.chars()
}

fn main() {
    let t = String::from("abc");
    println!("{:?}", chars_of(&t).collect::<Vec<_>>());
}
