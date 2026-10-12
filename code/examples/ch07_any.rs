// harness: run
//! A type map built on `Any` and `TypeId`: it can hold one value of any
//! `'static` type, and give it back with its type restored.
//!
//! Run with: `cargo run --example ch07_any`

use std::any::{Any, TypeId};
use std::collections::HashMap;

// ANCHOR: extensions
/// At most one value per type, like the "extensions" of an HTTP request.
#[derive(Default)]
struct Extensions {
    values: HashMap<TypeId, Box<dyn Any>>, // `Box<dyn Any + 'static>`
}

impl Extensions {
    fn insert<T: 'static>(&mut self, value: T) {
        self.values.insert(TypeId::of::<T>(), Box::new(value));
    }

    fn get<T: 'static>(&self) -> Option<&T> {
        self.values.get(&TypeId::of::<T>())?.downcast_ref::<T>()
    }
}
// ANCHOR_END: extensions

struct RequestId(u64);
struct UserName(String);

fn main() {
    // ANCHOR: main
    let mut extensions = Extensions::default();
    extensions.insert(RequestId(42));
    extensions.insert(UserName(String::from("ada")));
    extensions.insert("a literal"); // `&'static str` is a `'static` type

    let id: Option<&RequestId> = extensions.get::<RequestId>();
    let user: Option<&UserName> = extensions.get::<UserName>();
    println!("request id: {:?}", id.map(|id| id.0));
    println!("user name:  {:?}", user.map(|user| user.0.as_str()));
    println!("literal:    {:?}", extensions.get::<&'static str>());
    println!("String:     {:?}", extensions.get::<String>()); // never inserted
    // ANCHOR_END: main
}
