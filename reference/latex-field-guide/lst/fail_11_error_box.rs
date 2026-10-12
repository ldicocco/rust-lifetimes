// BadInput<'a> and its Display / Error impls: exactly as in the previous listing

fn parse(s: &str) -> Result<i32, Box<dyn Error>> {
    s.parse::<i32>().map_err(|_| BadInput { input: s }.into())
}

fn main() {
    println!("{:?}", parse("12x").is_err());
}
