use std::thread;

fn main() {
    let data = vec![1, 2, 3];
    let h = thread::spawn(|| println!("{data:?}")); // borrows `data`
    h.join().unwrap();
}
