// harness: run
//! A GAT as a "view" type: one source of configuration values lends borrowed
//! `&str`s, another builds owned `String`s, and generic code accepts both.
//!
//! Run with: `cargo run --example ch10_views`

// ANCHOR: trait
pub trait ConfigSource {
    /// What `get` returns when it borrows `self` for `'a`.
    type Value<'a>: AsRef<str>
    where
        Self: 'a;

    fn get(&self, key: &str) -> Option<Self::Value<'_>>;
}
// ANCHOR_END: trait

// ANCHOR: file_source
/// Values parsed from a file: the source owns them and lends `&str`s.
pub struct FileSource {
    entries: Vec<(String, String)>,
}

impl ConfigSource for FileSource {
    type Value<'a> = &'a str;

    fn get(&self, key: &str) -> Option<&str> {
        let (_, value) = self.entries.iter().find(|(k, _)| k == key)?;
        Some(value)
    }
}
// ANCHOR_END: file_source

// ANCHOR: computed_source
/// Values computed on demand: there is nothing to borrow, so they are owned.
pub struct Defaults {
    base_port: u16,
}

impl ConfigSource for Defaults {
    type Value<'a> = String;

    fn get(&self, key: &str) -> Option<String> {
        match key {
            "port" => Some(self.base_port.to_string()),
            "admin-port" => Some((self.base_port + 1).to_string()),
            _ => None,
        }
    }
}
// ANCHOR_END: computed_source

// ANCHOR: generic
/// Looks a key up in the first source, then in the second.
pub fn lookup<A, B>(first: &A, second: &B, key: &str) -> Option<String>
where
    A: ConfigSource,
    B: ConfigSource,
{
    if let Some(value) = first.get(key) {
        return Some(value.as_ref().to_owned());
    }
    second.get(key).map(|value| value.as_ref().to_owned())
}
// ANCHOR_END: generic

fn main() {
    // ANCHOR: main
    let file = FileSource {
        entries: vec![
            ("port".into(), "9000".into()),
            ("host".into(), "example.org".into()),
        ],
    };
    let defaults = Defaults { base_port: 8080 };
    for key in ["port", "admin-port", "host", "user"] {
        println!("{key:>10}: {:?}", lookup(&file, &defaults, key));
    }
    // ANCHOR_END: main
}
