// harness: run
//! In `static` and `const` items every elided lifetime is `'static`. Since Rust
//! 1.99 the same holds for associated consts, even in an `impl` that has lifetime
//! parameters of its own.
//!
//! Run with: `cargo run --example ch03_static_items`

// ANCHOR: items
// The compiler reads: static GREETING: &'static str
static GREETING: &str = "hello";
// The compiler reads: const STOP_WORDS: &'static [&'static str]
const STOP_WORDS: &[&str] = &["a", "an", "the"];
// ANCHOR_END: items

// ANCHOR: assoc
struct Tokenizer<'src> {
    text: &'src str,
}

impl<'src> Tokenizer<'src> {
    // The compiler reads: const DELIMITERS: &'static [char]
    const DELIMITERS: &[char] = &[' ', ',', ';'];

    fn words(&self) -> Vec<&'src str> {
        let pieces = self.text.split(Self::DELIMITERS);
        pieces
            .filter(|w| !w.is_empty() && !STOP_WORDS.contains(w))
            .collect()
    }

    // Compiles only because DELIMITERS is `'static`, not `'src`.
    fn delimiters() -> &'static [char] {
        Self::DELIMITERS
    }
}
// ANCHOR_END: assoc

fn main() {
    let text = String::from("the parser, an engine; a tokenizer");
    let tokenizer = Tokenizer { text: &text };
    println!("{GREETING}: {:?}", tokenizer.words());
    println!("delimiters: {:?}", Tokenizer::delimiters());
}
