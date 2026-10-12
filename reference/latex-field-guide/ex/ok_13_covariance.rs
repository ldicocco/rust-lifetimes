fn pick<'a>(a: &'a str, _b: &'a str) -> &'a str {
    a
}

fn main() {
    let s: &'static str = "static";
    let local = String::from("local");
    // &'static str is silently shrunk to the (shorter) lifetime of `local`.
    println!("{}", pick(s, &local));
}
