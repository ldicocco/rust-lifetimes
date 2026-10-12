// harness: compile-fail E0597
//! A function cannot return a future that borrows one of its own locals.

use std::future::Future;

async fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

// ANCHOR: make
pub fn count_config_words() -> impl Future<Output = usize> {
    let text = String::from("listen port 8080");
    count_words(&text)
}
// ANCHOR_END: make
