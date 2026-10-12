// harness: compile-fail "lifetime may not live long enough"
//! The closure passed to `LocalKey::with` receives a reference whose lifetime is
//! chosen inside `with` (it is higher-ranked): it cannot be returned.

// ANCHOR: escape
thread_local! {
    static GREETING: String = String::from("hello");
}

pub fn greeting() -> &'static str {
    GREETING.with(|text| text.as_str())
}
// ANCHOR_END: escape
