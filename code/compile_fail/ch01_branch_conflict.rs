// harness: compile-fail E0502
//! A use after the `if` keeps the borrow live on both paths, including the one
//! that mutates.

// ANCHOR: branch_conflict
pub fn describe_or_extend(words: &mut Vec<String>, verbose: bool) {
    let first = &words[0];
    if verbose {
        words.push(String::from("durations"));
    }
    println!("first word: {first}");
}
// ANCHOR_END: branch_conflict
