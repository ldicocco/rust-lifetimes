fn overwrite<'a>(slot: &mut &'a str, val: &'a str) {
    *slot = val;
}

fn main() {
    let mut s: &'static str = "static";
    {
        let local = String::from("local");
        overwrite(&mut s, &local); // would leave `s` dangling
    }
    println!("{s}");
}
