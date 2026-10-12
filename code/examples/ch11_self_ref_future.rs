// harness: run
//! An async block that keeps a reference to its own local across an `.await`
//! is self-referential. It may be moved before it is first polled; `block_on`
//! pins it, and from then on it stays put.
//!
//! Run with: `cargo run --example ch11_self_ref_future`

use std::future::Future;
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

fn main() {
    // ANCHOR: self_ref
    let task = async {
        let keys = ["listen", "port", "log-level"]; // stored in the future
        let first = &keys[0]; // points into the future itself
        yield_now().await; // `keys` and `first` are both kept across this point
        println!("first key: {first}");
    };
    // Before the first poll, the future is just data: moving it is fine.
    let mut queue = vec![task];
    let task = queue.pop().unwrap();
    block_on(task); // pins the future, then polls it until it is done
    // ANCHOR_END: self_ref
}
