// harness: run
//! An enum with a variant that borrows needs a lifetime parameter too, and the
//! parameter belongs to the type, whichever variant a value holds.
//!
//! Run with: `cargo run --example ch02_tokens`

// ANCHOR: tokens
#[derive(Debug)]
enum Token<'src> {
    Key(&'src str), // borrows from the input
    Number(u64),    // owns its data
    Equals,
}

enum LexError<'src> {
    BadWord { word: &'src str, position: usize },
}

fn tokenize<'src>(line: &'src str) -> Result<Vec<Token<'src>>, LexError<'src>> {
    let mut tokens = Vec::new();
    for (position, word) in line.split_whitespace().enumerate() {
        let token = if word == "=" {
            Token::Equals
        } else if let Ok(n) = word.parse() {
            Token::Number(n)
        } else if word.chars().all(|c| c.is_alphanumeric() || c == '_') {
            Token::Key(word)
        } else {
            return Err(LexError::BadWord { word, position });
        };
        tokens.push(token);
    }
    Ok(tokens)
}
// ANCHOR_END: tokens

// ANCHOR: main
fn main() {
    let config = String::from("port = 8080\nname = ???");
    for line in config.lines() {
        match tokenize(line) {
            Ok(tokens) => {
                let shown: Vec<String> = tokens
                    .iter()
                    .map(|token| match token {
                        Token::Key(key) => format!("key {key:?}"),
                        Token::Number(n) => format!("number {n}"),
                        Token::Equals => "equals".to_string(),
                    })
                    .collect();
                println!("{}", shown.join(", "));
            }
            Err(LexError::BadWord { word, position }) => {
                println!("bad word {word:?} at position {position}")
            }
        }
    }
    // A variant without a borrow still has the type's lifetime parameter,
    // which can be any lifetime at all, `'static` included.
    let always: Token<'static> = Token::Equals;
    println!("{always:?}");
}
// ANCHOR_END: main
