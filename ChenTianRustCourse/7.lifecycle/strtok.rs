fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();

    // 所以这里 hello 的生命周期和 s1 的可变引用的生命周期一致
    // 在 hello 使用完之前， 编译器会认为 s1 的可变引用一直存在
    let hello = strtok(&mut s1, ' ');

    println!("hello is: {}, s1: {}. s: {}", hello, s1, s);
}

/// 这里将 返回值的生命周期和 s1 的可变引用的生命周期绑定了
pub fn strtok<'a>(s: &'a mut &str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];

        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}