// `Box<dyn Fn..>` in a signature means `Box<dyn Fn.. + 'static>`.
fn make_prefixer(prefix: &str) -> Box<dyn Fn(&str) -> String> {
    Box::new(move |s| format!("{prefix}{s}"))
}

fn main() {
    let p = String::from(">> ");
    println!("{}", make_prefixer(&p)("hello"));
}
