use std::fmt;

#[derive(Debug, Default)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}

#[derive(Debug)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

impl Default for Language {
    fn default() -> Self {
        Language::Rust
    }
}

impl Developer {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            // 其他字段用 default
            ..Default::default()
        }
    }
}

fn main() {
    let dev1 = Developer::default();

    let dev2 = Developer::new("OK");

    println!("dev1: {:?}, dev2: {:?}", dev1, dev2);
}