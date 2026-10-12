// harness: run
//! In a loop, a borrow made in one iteration can be live at the start of the
//! next one, and dead again a few statements later.
//!
//! Run with: `cargo run --example ch01_loop`

// ANCHOR: loop
fn main() {
    let mut transcript = String::new();
    let mut previous: &str = "(start)";
    for word in ["regions", "not", "durations"] {
        println!("{previous} -> {word}"); // last use of the previous borrow
        if !transcript.is_empty() {
            transcript.push(' '); // `previous` is dead here: it is overwritten below
        }
        let start = transcript.len();
        transcript.push_str(word);
        previous = &transcript[start..]; // a new borrow of `transcript`
    }
    println!("last word: {previous}; transcript: {transcript}");
}
// ANCHOR_END: loop
