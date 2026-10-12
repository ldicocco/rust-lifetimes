// harness: compile-fail E0502
//! Use the previous iteration's borrow after the mutation and the borrow is
//! live across the `push_str` of the next iteration.

// ANCHOR: loop_conflict
pub fn caller() {
    let mut transcript = String::new();
    let mut previous: &str = "(start)";
    for word in ["regions", "not", "durations"] {
        let start = transcript.len();
        transcript.push_str(word);
        println!("{previous} -> {word}"); // uses the borrow made last time round
        previous = &transcript[start..];
    }
}
// ANCHOR_END: loop_conflict
