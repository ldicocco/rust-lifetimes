// The contract must be stated. Three honest choices:

// 1. Result may come from EITHER input: tie everything together.
fn either<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 2. Result comes only from `x`: say so; `y` stays independent.
fn first_only<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// 3. Result borrows from neither: no lifetime needed.
fn owned(x: &str, y: &str) -> String {
    format!("{x}{y}")
}

fn main() {
    let long = String::from("long-lived");
    let r;
    {
        let short = String::from("short");
        r = first_only(&long, &short); // fine: `r` does not depend on `short`
        println!("{}", either(&long, &short));
        println!("{}", owned(&long, &short));
    }
    println!("{r}");
}
