use std::thread;

fn main() {
    let data = vec![1, 2, 3];

    // 1. Ownership moves in: the closure is 'static.
    let owned = data.clone();
    thread::spawn(move || println!("moved: {owned:?}")).join().unwrap();

    // 2. Borrowing: scoped threads are joined before the scope ends.
    thread::scope(|s| {
        s.spawn(|| println!("sum = {}", data.iter().sum::<i32>()));
        s.spawn(|| println!("len = {}", data.len()));
    });
    println!("still mine: {data:?}");
}
