fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");

    let result = max(&s1, &s2);

    println!("bigger one: {}", result);
}

/// ERROR，编译器并不知道返回值的生命周期是与 s1 一致还是与 s2 一致
/// For more information about this error, try `rustc --explain E0106`.
// fn max(s1: &str, s2: &str) -> &str {
//     if s1 > s2 {
//         s1
//     } else {
//         s2
//     }
// }

fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}