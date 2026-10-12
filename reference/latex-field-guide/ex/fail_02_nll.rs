fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);           // needs &mut v while `first` is still live
    println!("{first}"); // <- this later use keeps the shared borrow alive
}
