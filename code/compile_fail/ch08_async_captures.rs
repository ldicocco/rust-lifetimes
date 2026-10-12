// harness: compile-fail E0597
//! The future returned by an `async fn` captures every input lifetime: it
//! borrows `key` until it is dropped.

// ANCHOR: async_key
pub trait Store {
    async fn fetch(&self, key: &str) -> Option<String>;
}

pub fn caller<S: Store>(store: &S) {
    let future;
    {
        let key = String::from("port");
        future = store.fetch(&key);
    } // `key` is dropped here
    drop(future);
}
// ANCHOR_END: async_key
