use std::error::Error;
use std::rc::Rc;

fn is_static<T: 'static>(_: &T) {}

fn plain(_: &str) {}

fn main() {
    is_static(&String::from("owned"));
    is_static(&vec![String::from("a")]);
    is_static(&"literal");                       // &'static str
    is_static(&Rc::new(String::from("rc")));     // Rc<T>: 'static iff T: 'static
    let e: Box<dyn Error> = "boom".into();       // Box<dyn Error + 'static>
    is_static(&e);
    is_static(&(plain as fn(&str)));             // fn pointers
    let owned = String::from("captured by move");
    is_static(&(move || owned.len()));           // move closure owning a String
}
