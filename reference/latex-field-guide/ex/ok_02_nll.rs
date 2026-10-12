fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    println!("{first}"); // last use of `first`: its lifetime ends here
    v.push(4);           // so this mutable borrow is accepted (NLL)
    println!("{v:?}");
}
