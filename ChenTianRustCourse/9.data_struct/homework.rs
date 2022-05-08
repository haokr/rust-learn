use std::borrow::Cow;


fn main() {
    let s1 = std::mem::size_of::<Cow<u8>>();
    let s2 = std::mem::size_of::<Cow<str>>();
    println!("s1: {}, s2: {}", s1, s2);
}