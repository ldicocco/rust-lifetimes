// harness: run
//! Rule 2 as rustc implements it: the elided output gets the lifetime of the only
//! parameter that mentions a lifetime, provided that this parameter mentions
//! exactly one lifetime (written or elided).
//!
//! Run with: `cargo run --example ch03_rule2_one_parameter`

use std::str::SplitWhitespace;

// ANCHOR: not_counted
// `index` has no lifetime, so `line` is the only parameter that mentions one.
fn field(line: &str, index: usize) -> &str {
    line.split(',').nth(index).unwrap_or("").trim()
}

// The `&str` inside `fn(&str) -> bool` belongs to the fn pointer type itself,
// which elides on its own: `pred` mentions no lifetime of `first_where`.
fn first_where(line: &str, pred: fn(&str) -> bool) -> &str {
    line.split_whitespace()
        .find(|word| pred(word))
        .unwrap_or("")
}

// `S` may well be a type that borrows (`&str` below), but elision only counts
// the lifetimes that are written in the signature.
fn last_field<S: AsRef<str>>(line: &str, separator: S) -> &str {
    line.rsplit(separator.as_ref()).next().unwrap_or("")
}
// ANCHOR_END: not_counted

// ANCHOR: hidden
// `SplitWhitespace` has a lifetime parameter; `'_` shows where it goes, and rule 2
// fills it in: fn words<'l>(line: &'l str) -> SplitWhitespace<'l>
fn words(line: &str) -> SplitWhitespace<'_> {
    line.split_whitespace()
}
// ANCHOR_END: hidden

// ANCHOR: named
// One parameter, one lifetime written twice: rule 2 applies.
#[allow(mismatched_lifetime_syntaxes)] // the lint is discussed below
fn first_of<'a>(pair: (&'a str, &'a str)) -> &str {
    pair.0
}

// A named lifetime counts like an elided one, `'static` included: the result is
// a `&'static str`.
#[allow(mismatched_lifetime_syntaxes)]
fn static_prefix(text: &'static str, len: usize) -> &str {
    &text[..len]
}
// ANCHOR_END: named

fn is_number(word: &str) -> bool {
    word.chars().all(|c| c.is_ascii_digit())
}

fn main() {
    let line = String::from("id, name , 42 ; extra");
    println!("field 1:     {:?}", field(&line, 1));
    println!(
        "first_where: {:?}",
        first_where("port 8080 open", is_number)
    );
    println!("last_field:  {:?}", last_field(&line, ";"));
    println!("words:       {}", words(&line).count());
    println!("first_of:    {:?}", first_of(("left", "right")));
    // The annotation compiles only because the elided output is `'static`.
    let prefix: &'static str = static_prefix("localhost", 5);
    println!("prefix:      {prefix:?}");
}
