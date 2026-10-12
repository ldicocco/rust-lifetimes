struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn requires_static<T: 'static>(_t: T) {
    println!("inside");
}

fn main() {
    requires_static(Noisy("a")); // Noisy: 'static -- yet it dies right here
    println!("after");
    requires_static(String::from("owned")); // local, owned: fine
    requires_static(7u8);
    requires_static("literal");
}
