// harness: compile-fail E0308
//! `F: Fn(&str) -> Fut` names one future type for every lifetime, but the
//! future of an `async fn` borrows its argument, so its type depends on it.

use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

fn block_on<T>(fut: impl Future<Output = T>) -> T {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

// ANCHOR: with_local
fn with_local<F, Fut>(f: F) -> usize
where
    F: Fn(&str) -> Fut,
    Fut: Future<Output = usize>,
{
    let local = String::from("created inside");
    block_on(f(&local))
}

async fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

pub fn caller() -> usize {
    with_local(count_words)
}
// ANCHOR_END: with_local
