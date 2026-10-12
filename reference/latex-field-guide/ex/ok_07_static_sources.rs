static BANNER: &str = "lifetimes";        // type is &'static str
const LIMITS: &[u32] = &[1, 2, 3];        // type is &'static [u32]

fn promoted() -> &'static i32 {
    &42                                   // constant promotion
}

fn leaked(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())         // heap data, never freed
}

fn main() {
    let l: &'static str = leaked(format!("{}-{}", "dyn", 7));
    println!("{BANNER} {LIMITS:?} {} {l}", promoted());
}
