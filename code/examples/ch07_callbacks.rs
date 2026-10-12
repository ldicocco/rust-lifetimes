// harness: run
//! A registry of callbacks stores `Box<dyn Fn(&str) -> String>`, which means
//! `Box<dyn Fn(&str) -> String + 'static>`: the callbacks must own what they use.
//!
//! Run with: `cargo run --example ch07_callbacks`

// ANCHOR: registry
#[derive(Default)]
struct Registry {
    handlers: Vec<Box<dyn Fn(&str) -> String>>, // `+ 'static` is implied
}

impl Registry {
    fn register(&mut self, handler: Box<dyn Fn(&str) -> String>) {
        self.handlers.push(handler);
    }

    fn run(&self, line: &str) {
        for handler in &self.handlers {
            println!("{}", handler(line));
        }
    }
}
// ANCHOR_END: registry

fn main() {
    // ANCHOR: main
    let mut registry = Registry::default();
    let prefix = String::from(">> ");
    registry.register(Box::new(move |line| format!("{prefix}{line}"))); // owns `prefix`
    registry.register(Box::new(|line| line.to_uppercase())); // captures nothing
    registry.run("hello");
    // ANCHOR_END: main
}
