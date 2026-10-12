fn main() {
    let b: Box<&'static str> = Box::new("static");
    let local = String::from("local");
    let mut shorter: Box<&str> = b; // Box<&'static str> -> Box<&'short str>: fine
    shorter = Box::new(&local);
    println!("{shorter}");
}
