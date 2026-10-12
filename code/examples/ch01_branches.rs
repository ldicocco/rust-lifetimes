// harness: run
//! A borrow can be live on one path through a function and dead on another.
//!
//! Run with: `cargo run --example ch01_branches`

// ANCHOR: branches
fn describe_or_extend(words: &mut Vec<String>) {
    let first = &words[0];
    if first.len() <= 5 {
        words.push(String::from("durations")); // `first` is dead on this path...
    } else {
        println!("first word is long: {first}"); // ...and used on this one
    }
}

fn main() {
    let mut short = vec![String::from("not")];
    let mut long = vec![String::from("regions")];
    describe_or_extend(&mut short);
    describe_or_extend(&mut long);
    println!("{short:?} {long:?}");
}
// ANCHOR_END: branches
