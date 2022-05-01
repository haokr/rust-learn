
pub trait Parse {
    fn parse(s: &str) -> Self;
}

impl Parse for u8 {
    fn parse(s: &str) -> Self {
        let mut number = String::from("");

        for elem in s.chars() {
            if elem >= '0' && elem <= '9' {
                number += &elem.to_string();
            }
        }
        if number.len() == 0 {
            return 0;
        }
        println!("number: {}", number);
        number.parse().unwrap()
    }
}

fn parse_should_work() {
    assert_eq!(u8::parse("123abc"), 123);
    assert_eq!(u8::parse("228abcd"), 0);
    assert_eq!(u8::parse("abc"), 0);
}

fn main() {
    parse_should_work();

    println!("result: {}", u8::parse("255 hello world"));
}