use std::str::FromStr;

pub trait Parse {
    type Error;
    fn parse(s: &str) -> Result<Self, Self::Error>
    where Self: Sized;
}

impl<T> Parse for T
where
    T: FromStr + Default,
{
    // 关联类型
    type Error = String;
    fn parse(s: &str) -> Result<Self, Self::Error> {
        let mut number = String::from("");

        for elem in s.chars() {
            if elem >= '0' && elem <= '9' {
                number += &elem.to_string();
            }
        }
        number.parse().map_err(|_err| "Failed to parse string".to_string())
    }
}

fn parse_should_work() { 
    assert_eq!(u32::parse("123abcd"), Ok(123)); 
    assert_eq!(f64::parse("2345abc"), Ok(2345.0));
}

fn main() {
    parse_should_work();
    println!("result: {}", f64::parse("255, hello world!").unwrap());
    println!("result: {}", u8::parse("255, hello world!").unwrap());
}