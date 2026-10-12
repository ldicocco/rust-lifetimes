// harness: compile-fail "implementation of `Parse` is not general enough"
//! A borrowing type implements `Parse<'src>` only for its own `'src`, so it does
//! not satisfy `for<'src> Parse<'src>`.

trait Parse<'src>: Sized {
    fn parse(src: &'src str) -> Option<Self>;
}

impl<'src> Parse<'src> for &'src str {
    fn parse(src: &'src str) -> Option<Self> {
        src.split_whitespace().next()
    }
}

fn parse_local<T>(bytes: &[u8]) -> Option<T>
where
    T: for<'src> Parse<'src>,
{
    let text = String::from_utf8_lossy(bytes).into_owned();
    T::parse(&text)
}

// ANCHOR: caller
pub fn caller() {
    let word: Option<&str> = parse_local(b"words");
    println!("{word:?}");
}
// ANCHOR_END: caller
