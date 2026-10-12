use std::thread;

fn spawn_drop<T: Send>(t: T) {
    thread::spawn(move || drop(t));
}

fn main() {
    spawn_drop(1);
}
