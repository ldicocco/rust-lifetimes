// harness: run
//! A higher-ranked bound that states a relationship the `Fn` sugar cannot
//! express: the result borrows from the first argument only.
//!
//! Run with: `cargo run --example ch09_relation`

// ANCHOR: split_head
fn split_head<F>(f: F) -> String
where
    F: for<'t> Fn(&'t str, &str) -> &'t str,
{
    let text = String::from("key=value");
    let head;
    {
        let sep = String::from("=");
        head = f(&text, &sep); // the result borrows `text`, not `sep`
    } // `sep` is dropped here; `head` is still valid
    head.to_string()
}
// ANCHOR_END: split_head

fn main() {
    // ANCHOR: call
    let head = split_head(|text, sep| text.split(sep).next().unwrap_or(text));
    // ANCHOR_END: call
    println!("{head}");
}
