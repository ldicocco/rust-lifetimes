// harness: compile-fail E0597
//! If `&dyn Plugin` meant `&'a (dyn Plugin + 'a)`, as the Reference's order of
//! rules says, every call would need a `'static` borrow: writing that type
//! explicitly shows what the default avoids.

pub trait Plugin: 'static {
    fn name(&self) -> String;
}

pub struct Upper;

impl Plugin for Upper {
    fn name(&self) -> String {
        "upper".to_string()
    }
}

// ANCHOR: explicit
pub fn describe<'a>(plugin: &'a (dyn Plugin + 'a)) -> String {
    format!("plugin {}", plugin.name())
}

pub fn caller() -> String {
    let local = Upper;
    describe(&local)
}
// ANCHOR_END: explicit
