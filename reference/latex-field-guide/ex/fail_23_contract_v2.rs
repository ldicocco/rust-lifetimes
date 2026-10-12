// Version 2: only the BODY changed (`x` -> `y`); the source of the signature is
// untouched, but a body-inferred contract would silently become:
fn pick<'a>(_x: &str, y: &'a str) -> &'a str {
    y
}

fn main() {
    let long = String::from("long-lived");
    let r;
    {
        let short = String::from("short");
        r = pick(&long, &short); // identical caller code
    }
    println!("{r}");
}
