// harness: run
//! An `async fn` returns a future that captures every input lifetime, and the
//! future holds those borrows until it completes. Run with the ten-line
//! `block_on` built on `Waker::noop`.
//!
//! Run with: `cargo run --example ch11_async_captures`

use std::collections::BTreeMap;
use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

// ANCHOR: block_on
/// Runs a future to completion on the current thread. Enough for futures that
/// never wait for anything external: they make progress at every poll.
fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut cx) {
            return output;
        }
    }
}
// ANCHOR_END: block_on

type Config = BTreeMap<String, String>;

// ANCHOR: lookup
fn lookup<'a>(config: &'a Config, key: &str) -> Option<&'a str> {
    config.get(key).map(String::as_str)
}

async fn lookup_async<'a>(config: &'a Config, key: &str) -> Option<&'a str> {
    println!("  (running lookup_async for {key:?})");
    config.get(key).map(String::as_str)
}
// ANCHOR_END: lookup

// ANCHOR: desugared
/// What the compiler makes of `lookup_async`: a plain function that returns an
/// opaque future capturing both lifetimes.
fn lookup_desugared<'a, 'k>(
    config: &'a Config,
    key: &'k str,
) -> impl Future<Output = Option<&'a str>> + use<'a, 'k> {
    async move {
        println!("  (running lookup_desugared for {key:?})");
        config.get(key).map(String::as_str)
    }
}
// ANCHOR_END: desugared

// ANCHOR: eager
/// Does the lookup before creating the future, so the future captures only
/// `'a`: `key` may be dropped as soon as this function returns.
fn lookup_eager<'a>(
    config: &'a Config,
    key: &str,
) -> impl Future<Output = Option<&'a str>> + use<'a> {
    let value = config.get(key).map(String::as_str);
    async move { value }
}
// ANCHOR_END: eager

async fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

// ANCHOR: owned
fn count_config_words() -> impl Future<Output = usize> {
    let text = String::from("listen port 8080");
    // The async block owns `text`, and lends it to `count_words` from inside.
    async move { count_words(&text).await }
}
// ANCHOR_END: owned

fn main() {
    let mut config = Config::new();
    config.insert("port".to_owned(), "8080".to_owned());

    // ANCHOR: lazy
    let key = String::from("port");
    let future = lookup_async(&config, &key);
    println!("future created; nothing has run yet");
    println!("result: {:?}", block_on(future));
    println!("result: {:?}", block_on(lookup_desugared(&config, &key)));
    // ANCHOR_END: lazy

    // ANCHOR: scoped_key
    let value = {
        let key = String::from("port");
        lookup(&config, &key) // the borrow of `key` ends with the call
    };
    let future = {
        let key = String::from("port");
        lookup_eager(&config, &key) // the future does not hold `key`
    };
    println!("sync: {value:?}, eager: {:?}", block_on(future));
    // ANCHOR_END: scoped_key

    println!("words: {}", block_on(count_config_words()));
}
