// harness: run
//! When the trait itself has a lifetime bound, that bound is the default, and it
//! wins over the container: `&dyn Plugin` is `&(dyn Plugin + 'static)` because
//! `Plugin: 'static`. For a trait such as `Source<'a>: 'a`, rustc uses `'a` in a
//! type alias (or with an early-bound `'a`), as the Reference says.
//!
//! Run with: `cargo run --example ch08_trait_bounds`

// ANCHOR: plugin
/// Plugins are stored in global registries, so the trait requires `'static`.
trait Plugin: 'static {
    fn name(&self) -> String;
}

struct Upper;

impl Plugin for Upper {
    fn name(&self) -> String {
        "upper".to_string()
    }
}

// `&dyn Plugin` is `&'_ (dyn Plugin + 'static)`: the trait's bound, not the
// reference's lifetime, is the default. The reference itself may be short.
fn describe(plugin: &dyn Plugin) -> String {
    format!("plugin {}", plugin.name())
}
// ANCHOR_END: plugin

// ANCHOR: supertrait
// A bound inherited from a supertrait counts too: `&dyn Loud` is
// `&'_ (dyn Loud + 'static)`.
trait Loud: Plugin {
    fn shout(&self) -> String {
        self.name().to_uppercase()
    }
}

impl Loud for Upper {}

fn announce(plugin: &dyn Loud) -> String {
    plugin.shout()
}
// ANCHOR_END: supertrait

// ANCHOR: source
/// A source of text that lives at least as long as `'a`.
trait Source<'a>: 'a {
    fn text(&self) -> &'a str;
}

impl<'a> Source<'a> for &'a str {
    fn text(&self) -> &'a str {
        self
    }
}

// In a type alias the trait's bound applies: this is `Box<dyn Source<'a> + 'a>`.
type BoxedSource<'a> = Box<dyn Source<'a>>;

fn boxed_source<'a>(text: &'a str) -> BoxedSource<'a> {
    Box::new(text)
}
// ANCHOR_END: source

// ANCHOR: main
fn main() {
    let local = Upper; // a local variable: the borrow below is short
    println!("{} / {}", describe(&local), announce(&local));

    let config = String::from("port = 8080");
    println!("{}", boxed_source(&config).text());
}
// ANCHOR_END: main
