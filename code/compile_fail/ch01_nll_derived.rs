// harness: compile-fail E0502
//! A borrow stays live as long as any reference derived from it may be used,
//! whatever variable or data structure holds it.

// ANCHOR: derived
pub fn caller() {
    let mut words = vec![String::from("regions")];
    let first = &words[0]; // last use of the variable `first`...
    let shortlist = vec![first]; // ...but the borrow now lives in `shortlist`
    words.push(String::from("durations"));
    println!("{shortlist:?}");
}
// ANCHOR_END: derived
