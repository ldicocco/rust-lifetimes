// harness: run
//! Reborrowing: a `&mut` reference is not `Copy`, yet you can pass the same one to
//! a function many times, because the compiler reborrows it (`&mut *words`).
//!
//! Run with: `cargo run --example ch05_reborrow`

// ANCHOR: implicit
fn add_word(words: &mut Vec<String>, word: &str) {
    words.push(word.to_owned());
}

fn collect(words: &mut Vec<String>) {
    add_word(words, "alpha"); // really `add_word(&mut *words, "alpha")`
    add_word(words, "beta"); // `words` is usable again: only a reborrow was passed
}
// ANCHOR_END: implicit

// ANCHOR: annotated
fn collect_more(words: &mut Vec<String>) {
    let other: &mut Vec<String> = words; // the type is written: a reborrow
    add_word(other, "gamma");
    add_word(words, "delta"); // fine: `other` is no longer used

    let explicit = &mut *words; // the same reborrow, spelled out
    explicit.push(String::from("epsilon"));
    words.sort(); // a method call reborrows its receiver too
}
// ANCHOR_END: annotated

// ANCHOR: generic
fn passthrough<T>(value: T) -> T {
    value
}

fn generic_call(words: &mut Vec<String>) {
    passthrough(&mut *words).push(String::from("zeta")); // explicit reborrow
    passthrough(&mut *words).push(String::from("eta"));
}
// ANCHOR_END: generic

// ANCHOR: returned
/// The result is a reborrow of `*words`: it may live as long as the caller's
/// borrow, but no longer.
fn longest_mut(words: &mut [String]) -> Option<&mut String> {
    words.iter_mut().max_by_key(|w| w.len())
}
// ANCHOR_END: returned

// ANCHOR: option
fn bump(counter: Option<&mut u32>) {
    if let Some(count) = counter {
        *count += 1;
    }
}

fn bump_twice(mut counter: Option<&mut u32>) {
    bump(counter.as_deref_mut()); // reborrow what is inside the `Option`
    bump(counter.as_deref_mut());
}
// ANCHOR_END: option

fn main() {
    let mut words = Vec::new();
    collect(&mut words);
    collect_more(&mut words);
    generic_call(&mut words);
    println!("{words:?}");

    if let Some(longest) = longest_mut(&mut words) {
        longest.make_ascii_uppercase();
    }
    println!("{words:?}");

    let mut count = 0;
    bump_twice(Some(&mut count));
    bump_twice(None);
    println!("count = {count}");
    assert_eq!(count, 2);
}
