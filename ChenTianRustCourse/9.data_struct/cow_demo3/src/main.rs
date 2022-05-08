use serde::Deserialize;
use std::borrow::Cow;
use serde_json;

#[derive(Debug, Deserialize)]
struct User<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    age: u8,
}


fn main() {
    let input = r#"{"name": "tyr", "age": 18}"#;
    let user:User = serde_json::from_str(input).unwrap();

    match user.name {
        Cow::Borrowed(x) => println!("borrowed {}", x),
        Cow::Owned(x) => println!("owned {}", x),
    }
}