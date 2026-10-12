// harness: compile-fail E0106
//! A trait method declaration has no body: there is nothing a body-inferring rule
//! could look at, yet every implementation must follow one contract.

// ANCHOR: trait_decl
pub trait Pick {
    fn pick(x: &str, y: &str) -> &str;
}
// ANCHOR_END: trait_decl
