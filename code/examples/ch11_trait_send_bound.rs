// harness: run
//! The fix for `Send` in traits on stable Rust: declare the method as returning
//! `impl Future + Send`. Implementations may still write `async fn`.
//!
//! Run with: `cargo run --example ch11_trait_send_bound`

use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Waker};

type Task = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

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
            self.tasks
                .retain_mut(|task| task.as_mut().poll(&mut cx).is_pending());
        }
    }
}

// ANCHOR: trait
pub trait Fetch {
    /// The returned future borrows `self` and `key`, and is `Send`.
    fn fetch(&self, key: &str) -> impl Future<Output = String> + Send;
}
// ANCHOR_END: trait

// ANCHOR: impls
pub struct FileSource {
    entries: BTreeMap<String, String>,
}

impl Fetch for FileSource {
    async fn fetch(&self, key: &str) -> String {
        self.entries.get(key).cloned().unwrap_or_default()
    }
}

pub struct Defaults;

impl Fetch for Defaults {
    async fn fetch(&self, key: &str) -> String {
        format!("<default {key}>")
    }
}
// ANCHOR_END: impls

// ANCHOR: spawn_fetch
pub fn spawn_fetch<T>(executor: &mut Executor, source: Arc<T>, key: &'static str)
where
    T: Fetch + Send + Sync + 'static,
{
    executor.spawn(async move {
        let value = source.fetch(key).await;
        println!("{key} = {value}");
    });
}
// ANCHOR_END: spawn_fetch

fn main() {
    let mut executor = Executor::default();
    let file = FileSource {
        entries: BTreeMap::from([("port".to_owned(), "9000".to_owned())]),
    };
    spawn_fetch(&mut executor, Arc::new(file), "port");
    spawn_fetch(&mut executor, Arc::new(Defaults), "host");
    executor.run();
}
