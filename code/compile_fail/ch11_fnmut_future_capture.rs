// harness: compile-fail "captured variable cannot escape `FnMut` closure body"
//! A closure that returns a future cannot lend the future its captured
//! state: `FnMut`'s output cannot borrow from the closure.

use std::future::Future;

// ANCHOR: for_each_line
pub async fn for_each_line<F, Fut>(text: &str, mut callback: F)
where
    F: FnMut(String) -> Fut,
    Fut: Future<Output = ()>,
{
    for line in text.lines() {
        callback(line.to_owned()).await;
    }
}

pub async fn collect_lines(text: &str) -> Vec<String> {
    let mut seen = Vec::new();
    for_each_line(text, |line| async {
        seen.push(line);
    })
    .await;
    seen
}
// ANCHOR_END: for_each_line
