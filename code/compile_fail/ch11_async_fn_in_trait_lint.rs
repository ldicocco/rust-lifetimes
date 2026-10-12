// harness: compile-fail "use of `async fn` in public traits is discouraged"
//! The warn-by-default lint `async_fn_in_trait`, made an error so that the
//! harness can show it.
#![deny(async_fn_in_trait)]

// ANCHOR: trait
pub trait Fetch {
    async fn fetch(&self, key: &str) -> String;
}
// ANCHOR_END: trait
