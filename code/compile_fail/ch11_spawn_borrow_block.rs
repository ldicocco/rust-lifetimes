// harness: compile-fail E0373
//! An async block that borrows a local cannot be spawned: the executor may
//! poll it after the function has returned.

use std::future::Future;
use std::pin::Pin;

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
}

async fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

// ANCHOR: spawn
pub fn spawn_count(executor: &mut Executor) {
    let config = String::from("listen port 8080");
    executor.spawn(async {
        println!("{} words", count_words(&config).await);
    });
}
// ANCHOR_END: spawn
