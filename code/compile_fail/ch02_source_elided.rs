// harness: compile-fail E0499
//! Without a lifetime parameter on the trait, a piece can only borrow from the
//! `&mut self` borrow of the source, and pieces cannot be collected.

// ANCHOR: elided
pub trait Source {
    fn next_piece(&mut self) -> Option<&str>; // borrows from `&mut self`
}

pub fn collect_all<S: Source>(source: &mut S) -> Vec<&str> {
    let mut pieces = Vec::new();
    while let Some(piece) = source.next_piece() {
        pieces.push(piece);
    }
    pieces
}
// ANCHOR_END: elided
