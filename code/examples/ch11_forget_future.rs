// harness: run
//! Forgetting a future is safe, and its destructors never run. That is why an
//! async API cannot rely on a future's `Drop` to wait for borrowed data.
//!
//! Run with: `cargo run --example ch11_forget_future`

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

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

// ANCHOR: session
/// Stands for any resource with cleanup: a connection, a lock, a running task.
struct Connection {
    name: &'static str,
}

impl Drop for Connection {
    fn drop(&mut self) {
        println!("closing {}", self.name);
    }
}

async fn session(name: &'static str) {
    let _connection = Connection { name };
    yield_now().await; // suspended here, holding the connection
    println!("{name} finished");
}
// ANCHOR_END: session

fn main() {
    // ANCHOR: main
    let mut cx = Context::from_waker(Waker::noop());

    let mut dropped = Box::pin(session("dropped"));
    assert!(dropped.as_mut().poll(&mut cx).is_pending());
    drop(dropped); // runs the destructors of the suspended future

    let mut forgotten = Box::pin(session("forgotten"));
    assert!(forgotten.as_mut().poll(&mut cx).is_pending());
    std::mem::forget(forgotten); // safe: no destructor will ever run

    println!("end of main");
    // ANCHOR_END: main
}
