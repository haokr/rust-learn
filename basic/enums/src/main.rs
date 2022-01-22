fn main() {
    enum_demo3();
    option_demo1();
}

fn enum_demo() {
    let v4 = IpAddressKind::V4;
    let v6 = IpAddressKind::V6;
}

fn enum_demo2() {
    let v4 = IpAddressKind::V4(String::from("127.0.0.1"));
    let v6 = IpAddressKind::V6(String::from("::1"));
}

fn enum_demo3() {
    let m = Message::Write(String::from("Hi, this is a message."));
    m.call();
}

fn option_demo1() {
    let some_number = Some(3);
    let absent_number: Option<i32> = None;

    println!("{:?}", some_number);
    println!("{:?}", absent_number);
}

enum IpAddressKind {
    V4(String),
    V6(String),
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        dbg!(self);
    }
}

// enum Option<T> {
//     None,
//     Some(T)
// }