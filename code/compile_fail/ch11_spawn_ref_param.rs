// harness: compile-fail E0521
//! `async move` moves what the block captures; when that is a reference, the
//! future still borrows.

use std::future::Future;

pub fn spawn<F>(task: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    drop(task);
}

// ANCHOR: spawn
pub fn spawn_report(config: &str) {
    spawn(async move {
        println!("{} words", config.split_whitespace().count());
    });
}
// ANCHOR_END: spawn
