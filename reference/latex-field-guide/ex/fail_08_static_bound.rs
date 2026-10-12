fn requires_static<T: 'static>(_t: T) {}

fn main() {
    let local = String::from("x");
    requires_static(&local); // &'a String: 'static  only if 'a == 'static
}
