use std::cell::Cell;

fn main() {
    let c: Cell<&'static str> = Cell::new("static");
    let local = String::from("local");
    let r: &Cell<&str> = &c; // Cell<&'static str> is NOT a Cell<&'short str>
    r.set(&local);
}
