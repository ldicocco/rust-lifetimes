// harness: compile-fail "future cannot be sent between threads safely"
//! Generic code cannot know whether the future returned by an `async fn` in a
//! trait is `Send`.

use std::future::Future;
use std::sync::Arc;

pub fn spawn<F>(task: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    drop(task);
}

// ANCHOR: trait
pub trait Fetch {
    async fn fetch(&self, key: &str) -> String;
}
// ANCHOR_END: trait

// ANCHOR: spawn_fetch
pub fn spawn_fetch<T>(source: Arc<T>)
where
    T: Fetch + Send + Sync + 'static,
{
    spawn(async move {
        let value = source.fetch("port").await;
        println!("port = {value}");
    });
}
// ANCHOR_END: spawn_fetch
