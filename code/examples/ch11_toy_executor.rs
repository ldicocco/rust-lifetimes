// harness: run
//! A toy executor whose `spawn` has the signature of real executors:
//! `F: Future + Send + 'static`. Tasks own their data and lend it to the
//! futures they await.
//!
//! Run with: `cargo run --example ch11_toy_executor`

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

// ANCHOR: yield_now
/// A future that is pending the first time it is polled, and ready the
/// second time: a cooperative "let the others run".
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
// ANCHOR_END: yield_now

// ANCHOR: executor
type Task = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

/// Keeps spawned tasks and polls them in turn until all are done.
#[derive(Default)]
pub struct Executor {
    tasks: Vec<Task>,
}

impl Executor {
    pub fn spawn<F>(&mut self, task: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.tasks.push(Box::pin(task));
    }

    pub fn run(&mut self) {
        let mut cx = Context::from_waker(Waker::noop());
        while !self.tasks.is_empty() {
            // Poll every task once; keep the ones that are still pending.
            self.tasks
                .retain_mut(|task| task.as_mut().poll(&mut cx).is_pending());
        }
    }
}
// ANCHOR_END: executor

// ANCHOR: count_words
async fn count_words(label: &str, text: &str) -> usize {
    let mut count = 0;
    for word in text.split_whitespace() {
        println!("{label} reads {word:?}");
        count += 1;
        yield_now().await;
    }
    count
}
// ANCHOR_END: count_words

fn main() {
    // ANCHOR: spawn
    let mut executor = Executor::default();
    let config: Arc<str> = Arc::from("listen port 8080");
    for label in ["first", "second"] {
        let config = Arc::clone(&config); // each task owns a handle
        executor.spawn(async move {
            let count = count_words(label, &config).await;
            println!("{label} counted {count} words");
        });
    }
    executor.run();
    // ANCHOR_END: spawn
}
