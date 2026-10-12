// harness: run
//! `AsyncFn(&str) -> usize` (Rust 1.85): a higher-ranked bound whose future may
//! borrow the argument. Runs with a tiny executor built from the standard
//! library only.
//!
//! Run with: `cargo run --example ch09_async_fn`

use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

// ANCHOR: block_on
/// Polls a future to completion on the current thread. Good enough for futures
/// that never wait on anything external, like the ones in this example.
fn block_on<T>(fut: impl Future<Output = T>) -> T {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
    }
}
// ANCHOR_END: block_on

// ANCHOR: with_local
fn with_local<F>(f: F) -> usize
where
    F: AsyncFn(&str) -> usize,
{
    let local = String::from("created inside");
    block_on(f(&local))
}

async fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}
// ANCHOR_END: with_local

fn main() {
    // ANCHOR: calls
    let bonus = 100;
    println!("{}", with_local(count_words));
    println!("{}", with_local(async |s: &str| s.len() + bonus));
    // ANCHOR_END: calls
}
