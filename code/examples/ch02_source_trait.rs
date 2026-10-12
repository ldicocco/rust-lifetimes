// harness: run
//! A trait with a lifetime parameter: what `next_piece` returns borrows for
//! `'a`, which is independent of the `&mut self` borrow.
//!
//! Run with: `cargo run --example ch02_source_trait`

// ANCHOR: trait
/// Something that hands out pieces of text borrowed for `'a`.
trait Source<'a> {
    fn next_piece(&mut self) -> Option<&'a str>;
}
// ANCHOR_END: trait

// ANCHOR: fields
/// The comma-separated fields of a borrowed line.
struct Fields<'src> {
    rest: Option<&'src str>,
}

impl<'src> Source<'src> for Fields<'src> {
    fn next_piece(&mut self) -> Option<&'src str> {
        let rest = self.rest?;
        match rest.split_once(',') {
            Some((head, tail)) => {
                self.rest = Some(tail);
                Some(head.trim())
            }
            None => {
                self.rest = None;
                Some(rest.trim())
            }
        }
    }
}
// ANCHOR_END: fields

// ANCHOR: defaults
/// Hands out string literals, so it is a `Source<'a>` for every `'a`.
struct Defaults {
    left: usize,
}

impl<'a> Source<'a> for Defaults {
    fn next_piece(&mut self) -> Option<&'a str> {
        if self.left == 0 {
            return None;
        }
        self.left -= 1;
        Some("default")
    }
}
// ANCHOR_END: defaults

// ANCHOR: collect
/// The pieces borrow from the source's data, not from `source` itself.
fn collect_all<'a, S: Source<'a>>(source: &mut S) -> Vec<&'a str> {
    let mut pieces = Vec::new();
    while let Some(piece) = source.next_piece() {
        pieces.push(piece);
    }
    pieces
}

fn main() {
    let line = String::from("name, port, host");
    let mut fields = Fields { rest: Some(&line) };
    let pieces = collect_all(&mut fields);
    let mut defaults = Defaults { left: 2 };
    let more = collect_all(&mut defaults);
    println!("{pieces:?} {more:?}");
}
// ANCHOR_END: collect
