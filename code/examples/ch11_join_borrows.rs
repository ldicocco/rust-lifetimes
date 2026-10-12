// harness: run
//! Concurrency inside one task may borrow freely: the child futures live
//! inside the future that runs them, and nobody else can poll them.
//!
//! Run with: `cargo run --example ch11_join_borrows`

use std::future::{Future, poll_fn};
use std::pin::{Pin, pin};
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

struct YieldNow {
    yielded: bool,
}

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            Poll::Pending
        }
    }
}

fn yield_now() -> YieldNow {
    YieldNow { yielded: false }
}

// ANCHOR: join
/// Polls two futures in turn until both are done.
async fn join<A: Future, B: Future>(a: A, b: B) -> (A::Output, B::Output) {
    let mut a = pin!(a);
    let mut b = pin!(b);
    let (mut out_a, mut out_b) = (None, None);
    poll_fn(|cx| {
        if out_a.is_none()
            && let Poll::Ready(value) = a.as_mut().poll(cx)
        {
            out_a = Some(value);
        }
        if out_b.is_none()
            && let Poll::Ready(value) = b.as_mut().poll(cx)
        {
            out_b = Some(value);
        }
        if out_a.is_some() && out_b.is_some() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    })
    .await;
    (out_a.unwrap(), out_b.unwrap())
}
// ANCHOR_END: join

async fn count_words(text: &str) -> usize {
    let mut count = 0;
    for word in text.split_whitespace() {
        println!("words: {word:?}");
        count += 1;
        yield_now().await;
    }
    count
}

async fn count_lines(text: &str) -> usize {
    let mut count = 0;
    for line in text.lines() {
        println!("lines: {line:?}");
        count += 1;
        yield_now().await;
    }
    count
}

fn main() {
    // ANCHOR: main
    let config = String::from("listen 8080\nlog-level debug");
    // Both futures borrow `config`, a local; no `'static` is needed.
    let (words, lines) = block_on(join(count_words(&config), count_lines(&config)));
    println!("{words} words, {lines} lines");
    // ANCHOR_END: main
}
