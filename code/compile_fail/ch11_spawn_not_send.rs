// harness: compile-fail "future cannot be sent between threads safely"
//! A future that holds an `Rc` across an `.await` is not `Send`.

use std::future::Future;
use std::rc::Rc;

pub fn spawn<F>(task: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    drop(task);
}

async fn tick() {}

// ANCHOR: spawn
pub fn spawn_shared() {
    spawn(async {
        let shared = Rc::new(String::from("listen port 8080"));
        tick().await;
        println!("{shared}");
    });
}
// ANCHOR_END: spawn
