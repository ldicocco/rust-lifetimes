// harness: run
//! When temporaries are dropped: at the end of the statement, at the end of the
//! block when lifetime-extended, after the arms of a `match`, before the `else` of
//! an `if let` (edition 2024), and before the locals of a block when they come from
//! its tail expression (edition 2024).
//!
//! Run with: `cargo run --example ch05_temporaries`

// ANCHOR: noisy
/// A value that announces its creation and its destruction.
struct Noisy(&'static str);

fn make(name: &'static str) -> Noisy {
    println!("  make {name}");
    Noisy(name)
}

impl Noisy {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn find(&self, c: char) -> Option<usize> {
        self.0.find(c)
    }
}

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  drop {}", self.0);
    }
}
// ANCHOR_END: noisy

// ANCHOR: tail
fn tail_expression() -> usize {
    let _local = make("local");
    make("tail").len() // edition 2024: `tail` is dropped before `_local`
}
// ANCHOR_END: tail

fn main() {
    // ANCHOR: statement
    println!("1. end of statement");
    let len = make("a").len();
    println!("  after the let, len = {len}");
    // ANCHOR_END: statement

    // ANCHOR: extended
    println!("2. lifetime extension");
    {
        let held = &make("b");
        println!("  after the let, still holding {}", held.0);
    }
    println!("  after the block");
    // ANCHOR_END: extended

    // ANCHOR: match
    println!("3. match scrutinee");
    match make("c").len() {
        n => println!("  in the arm, n = {n}"),
    }
    println!("  after the match");
    // ANCHOR_END: match

    // ANCHOR: if_let
    println!("4. if let scrutinee");
    if let Some(i) = make("dog").find('o') {
        println!("  in the then-branch, i = {i}");
    }
    if let Some(i) = make("cat").find('o') {
        println!("  in the then-branch, i = {i}");
    } else {
        println!("  in the else-branch");
    }
    // ANCHOR_END: if_let

    println!("5. tail expression");
    let n = tail_expression();
    println!("  returned {n}");
}
