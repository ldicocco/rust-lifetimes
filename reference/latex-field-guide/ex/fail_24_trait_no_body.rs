// A trait method declaration has no body, so there is nothing to infer from.
trait Pick {
    fn pick(x: &str, y: &str) -> &str;
}

fn main() {}
