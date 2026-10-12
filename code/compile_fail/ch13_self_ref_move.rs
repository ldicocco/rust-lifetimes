// harness: compile-fail E0505
//! Built in place, a self-referential value is accepted, but it is borrowed by
//! itself: it can never be moved (or mutated) again.

pub struct Document<'a> {
    text: String,
    title: &'a str,
}

// ANCHOR: in_place
pub fn load() {
    let mut doc = Document {
        text: String::from("Config\nport 8080"),
        title: "",
    };
    doc.title = doc.text.lines().next().unwrap_or("");
    println!("{}", doc.title); // fine: used in place
    let moved = doc; // a move would leave `title` pointing at the old place
    println!("{}", moved.title);
}
// ANCHOR_END: in_place
