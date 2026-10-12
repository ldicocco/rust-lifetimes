// harness: run
//! `impl Trait` in traits (RPITIT) and `async fn` in traits capture every generic
//! parameter in scope, `Self` and the lifetime of `&self` included, in every
//! edition. `use<..>` (Rust 1.87 in traits) narrows the captures; it must list
//! `Self` and the trait's other type parameters.
//!
//! Run with: `cargo run --example ch08_rpitit`

use std::collections::BTreeMap;
use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

// ANCHOR: inventory
trait Inventory {
    // The ids are copied out: the iterator does not borrow `self`.
    fn ids(&self) -> impl Iterator<Item = u32> + use<Self>;
}

struct Shelf {
    items: Vec<u32>,
}

impl Inventory for Shelf {
    // `Shelf` has no generic parameters, so the impl captures nothing.
    fn ids(&self) -> impl Iterator<Item = u32> + use<> {
        self.items.clone().into_iter()
    }
}

fn restock<I: Inventory>(inventory: &mut I, add: impl FnOnce(&mut I)) -> u32 {
    let before = inventory.ids(); // does not keep `inventory` borrowed...
    add(inventory); // ...so it can be borrowed mutably here
    before.sum()
}
// ANCHOR_END: inventory

// ANCHOR: store
trait Store {
    // The future borrows `self` and `key` until it completes.
    async fn fetch(&self, key: &str) -> Option<String>;

    // The future borrows `self` only: `'_` is the lifetime of `&self`.
    fn fetch_owned(
        &self,
        key: &str,
    ) -> impl Future<Output = Option<String>> + use<'_, Self>;
}

struct Memory {
    map: BTreeMap<String, String>,
}

impl Store for Memory {
    async fn fetch(&self, key: &str) -> Option<String> {
        self.map.get(key).cloned()
    }

    fn fetch_owned(&self, key: &str) -> impl Future<Output = Option<String>> + use<'_> {
        let key = key.to_owned(); // copied before the future is created
        async move { self.map.get(&key).cloned() }
    }
}
// ANCHOR_END: store

/// Polls a future once. Enough here, because these futures never wait.
fn poll_once<F: Future>(future: F) -> Option<F::Output> {
    let mut context = Context::from_waker(Waker::noop());
    match pin!(future).poll(&mut context) {
        Poll::Ready(value) => Some(value),
        Poll::Pending => None,
    }
}

// ANCHOR: main
fn main() {
    let mut shelf = Shelf {
        items: vec![1, 2, 3],
    };
    let sum = restock(&mut shelf, |s| s.items.push(4));
    println!("sum before restocking: {sum}, items now: {:?}", shelf.items);

    let store = Memory {
        map: BTreeMap::from([("port".into(), "8080".into())]),
    };
    let future;
    {
        let key = String::from("port");
        future = store.fetch_owned(&key);
    } // `key` is dropped here: the future does not borrow it
    println!("{:?}", poll_once(future));
    println!("{:?}", poll_once(store.fetch("host")));
}
// ANCHOR_END: main
