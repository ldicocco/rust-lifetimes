// harness: compile-fail E0502
//! A future holds the borrows of its arguments until it completes, not until
//! the call returns.

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

// ANCHOR: holds
async fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

pub fn caller() -> usize {
    let mut text = String::from("listen port 8080");
    let future = count_words(&text);
    text.push_str(" log-level debug");
    block_on(future)
}
// ANCHOR_END: holds
