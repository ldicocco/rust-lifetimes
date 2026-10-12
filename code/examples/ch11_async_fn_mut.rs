// harness: run
//! `AsyncFnMut`: an async callback that borrows its argument and mutates its
//! captured state across an `.await`.
//!
//! Run with: `cargo run --example ch11_async_fn_mut`

use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut cx) {
            return output;
        }
    }
}

// ANCHOR: for_each_line
async fn for_each_line(text: &str, mut callback: impl AsyncFnMut(&str)) {
    for line in text.lines() {
        callback(line).await;
    }
}
// ANCHOR_END: for_each_line

async fn normalize(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn main() {
    // ANCHOR: main
    let config = String::from("listen   port 8080\n  log-level  debug");
    let mut seen = Vec::new();
    let collect = async |line: &str| {
        let normalized = normalize(line).await; // borrows `line` across .await
        seen.push(normalized); // mutates the captured `seen`
    };
    block_on(for_each_line(&config, collect));
    println!("{seen:?}");
    // ANCHOR_END: main
}
