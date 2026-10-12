// harness: compile-fail E0597
//! The bound rustc suggests for `Fn(&str, &str) -> &str` ties both inputs to the
//! output, so the result can no longer outlive the separator.

// ANCHOR: split_head
pub fn split_head<F>(f: F) -> String
where
    F: for<'a> Fn(&'a str, &'a str) -> &'a str, // the compiler's suggestion
{
    let text = String::from("key=value");
    let head;
    {
        let sep = String::from("=");
        head = f(&text, &sep);
    }
    head.to_string()
}
// ANCHOR_END: split_head
