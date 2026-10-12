fn chars_of(s: &str) -> impl Iterator<Item = char> + '_ {
    s.chars()
}

fn main() {
    let t = String::from("abc");
    println!("{:?}", chars_of(&t).collect::<Vec<_>>());
}
