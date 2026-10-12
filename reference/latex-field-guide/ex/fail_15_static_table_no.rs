struct Holder<'a>(&'a str);

fn is_static<T: 'static>(_: &T) {}

fn main() {
    let s = String::from("local");
    is_static(&s.as_str());          // &'a str with a local 'a
    is_static(&Holder(&s));          // struct carrying a lifetime parameter
    is_static(&(|| s.len()));        // closure borrowing `s`
}
