use std::thread;

fn spawn_drop<T: Send + 'static>(t: T) {
    thread::spawn(move || drop(t)).join().unwrap();
}

fn main() {
    spawn_drop(String::from("ok"));
}
