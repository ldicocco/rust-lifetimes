// harness: compile-fail E0277
//! An async block is not `Unpin`: it can only be polled through a pinned
//! pointer, which promises that it will not move again.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

// ANCHOR: poll_once
pub fn poll_once<F: Future + Unpin>(future: &mut F) -> Poll<F::Output> {
    let mut cx = Context::from_waker(Waker::noop());
    Pin::new(future).poll(&mut cx)
}
// ANCHOR_END: poll_once

async fn tick() {}

// ANCHOR: caller
pub fn caller() {
    let mut task = async {
        let keys = ["listen", "port"];
        let first = &keys[0];
        tick().await;
        println!("{first}");
    };
    let _ = poll_once(&mut task);
}
// ANCHOR_END: caller
