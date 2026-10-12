// harness: compile-fail E0597
//! The future of an `async fn` captures every input lifetime, even one that
//! does not appear in its output type.

use std::collections::BTreeMap;
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

type Config = BTreeMap<String, String>;

async fn lookup_async<'a>(config: &'a Config, key: &str) -> Option<&'a str> {
    config.get(key).map(String::as_str)
}

// ANCHOR: caller
pub fn caller(config: &Config) -> Option<&str> {
    let future = {
        let key = String::from("port");
        lookup_async(config, &key)
    };
    block_on(future)
}
// ANCHOR_END: caller
