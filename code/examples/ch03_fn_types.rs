// harness: run
//! Elision outside plain functions: fn pointer types and `Fn` sugar elide on
//! their own (even inside a struct field), and every elided lifetime in an `impl`
//! header becomes a fresh parameter of the `impl`.
//!
//! Run with: `cargo run --example ch03_fn_types`

// ANCHOR: pipeline
// Each fn type below is a small signature with its own elision:
//   fn(&str) -> &str             is  for<'a> fn(&'a str) -> &'a str
//   dyn Fn(&str) -> Vec<&str>    is  dyn for<'a> Fn(&'a str) -> Vec<&'a str>
struct Pipeline {
    normalize: fn(&str) -> &str,
    split: Box<dyn Fn(&str) -> Vec<&str>>,
}

impl Pipeline {
    fn run<'t>(&self, text: &'t str) -> Vec<&'t str> {
        (self.split)((self.normalize)(text))
    }
}
// ANCHOR_END: pipeline

// ANCHOR: impl_header
trait Describe {
    fn describe(&self) -> String;
}

// The compiler reads: impl<'a> Describe for &'a str
impl Describe for &str {
    fn describe(&self) -> String {
        format!("{} bytes of text", self.len())
    }
}
// ANCHOR_END: impl_header

fn main() {
    let pipeline = Pipeline {
        normalize: str::trim,
        split: Box::new(|s| s.split(',').collect()),
    };
    let input = String::from("  red,green,blue  ");
    println!("{:?}", pipeline.run(&input));
    println!("{}", input.as_str().describe());
}
