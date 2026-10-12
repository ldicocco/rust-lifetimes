// harness: compile-fail E0658
//! Return type notation, `T::fetch(..): Send`, is unstable in Rust 1.99.

use std::future::Future;
use std::sync::Arc;

pub fn spawn<F>(task: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    drop(task);
}

#[allow(async_fn_in_trait)]
pub trait Fetch {
    async fn fetch(&self, key: &str) -> String;
}

// ANCHOR: rtn
pub fn spawn_fetch<T>(source: Arc<T>)
where
    T: Fetch + Send + Sync + 'static,
    T::fetch(..): Send,
{
    spawn(async move {
        let value = source.fetch("port").await;
        println!("port = {value}");
    });
}
// ANCHOR_END: rtn
