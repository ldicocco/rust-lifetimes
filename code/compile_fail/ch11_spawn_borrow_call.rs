// harness: compile-fail E0597
//! The future returned by an `async fn` call borrows its arguments, so it is
//! not `'static` when an argument borrows a local.

use std::future::Future;

pub fn spawn<F>(task: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    drop(task); // a real executor would store the task and poll it later
}

// ANCHOR: spawn
async fn report(text: &str) {
    println!("{} words", text.split_whitespace().count());
}

pub fn spawn_report() {
    let config = String::from("listen port 8080");
    spawn(report(&config));
}
// ANCHOR_END: spawn
