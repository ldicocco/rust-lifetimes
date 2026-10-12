// harness: run
//! A trait with a lifetime parameter, like serde's `Deserialize<'de>`, and the
//! two ways to bound it: a caller-chosen `'src`, or `for<'src>`.
//!
//! Run with: `cargo run --example ch09_trait_param`

// ANCHOR: trait
/// Parses a value from text that lives for `'src`.
trait Parse<'src>: Sized {
    fn parse(src: &'src str) -> Option<Self>;
}

// A borrowing implementation: the result points into the source text.
impl<'src> Parse<'src> for &'src str {
    fn parse(src: &'src str) -> Option<Self> {
        src.split_whitespace().next()
    }
}

// An owning implementation: it works for any source lifetime.
impl<'src> Parse<'src> for u32 {
    fn parse(src: &'src str) -> Option<Self> {
        src.trim().parse().ok()
    }
}
// ANCHOR_END: trait

// ANCHOR: functions
/// The caller provides the text, so the result may borrow from it.
fn parse_from<'src, T: Parse<'src>>(src: &'src str) -> Option<T> {
    T::parse(src)
}

/// The text is a local buffer, so `T` must be parseable from a borrow of any
/// lifetime: in practice, `T` cannot borrow from it.
fn parse_local<T>(bytes: &[u8]) -> Option<T>
where
    T: for<'src> Parse<'src>,
{
    let text = String::from_utf8_lossy(bytes).into_owned();
    T::parse(&text)
}
// ANCHOR_END: functions

fn main() {
    // ANCHOR: calls
    let text = String::from("borrowed words");
    let word: Option<&str> = parse_from(&text);
    let n: Option<u32> = parse_from(" 7 ");
    let m: Option<u32> = parse_local(b" 42 ");
    println!("{word:?} {n:?} {m:?}");
    // ANCHOR_END: calls
}
