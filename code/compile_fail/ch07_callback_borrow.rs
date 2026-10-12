// harness: compile-fail E0597
//! `Box<dyn Fn(&str) -> String>` has a hidden `+ 'static`, so a boxed closure
//! that borrows a local variable cannot be stored in it.

// ANCHOR: borrow
pub struct Registry {
    handlers: Vec<Box<dyn Fn(&str) -> String>>,
}

impl Registry {
    pub fn register(&mut self, handler: Box<dyn Fn(&str) -> String>) {
        self.handlers.push(handler);
    }
}

pub fn caller(registry: &mut Registry) {
    let prefix = String::from(">> ");
    registry.register(Box::new(|line| format!("{prefix}{line}"))); // borrows `prefix`
}
// ANCHOR_END: borrow
