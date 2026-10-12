fn make_prefixer(prefix: &str) -> Box<dyn Fn(&str) -> String + '_> {
    Box::new(move |s| format!("{prefix}{s}"))
}

fn main() {
    let p = String::from(">> ");
    let f = make_prefixer(&p);
    println!("{}", f("hello"));
}
