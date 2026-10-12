// Hypothetical rule C: "infer the contract from the body".
// Version 1 of a library function returns `x`; the contract is therefore:
fn pick<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

fn main() {
    let long = String::from("long-lived");
    let r;
    {
        let short = String::from("short");
        r = pick(&long, &short); // caller relies on: result does not borrow `short`
    }
    println!("{r}");
}
