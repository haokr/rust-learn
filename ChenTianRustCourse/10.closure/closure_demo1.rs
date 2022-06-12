use std::{collections::HashMap, mem::size_of_val};

/// 不带 move 时，闭包捕获的是变量的引用
/// 带 move 时，对应自由变量的所有权会被转移到闭包中
/// 闭包的大小跟参数、局部变量都无关，只跟捕获的变量有关，因为他们是在调用时才在栈上产生内存分配
/// 
/// 闭包产生的匿名数据类型，格式和 struct 是一样的
/// 闭包是存储在栈上，并且除了捕获的数据外，闭包本身不包含任何额外函数指针指向闭包的代码
fn main() {
    // 长度为 0 
    let c1 = || println!("hello world!");
    // 与参数无关，长度也为 0
    let c2 = |i: u32| println!(
        "helloL {}", i
    );

    let name = String::from("tyr");
    let name1 = name.clone();

    let mut table = HashMap::new();
    table.insert("hello", "world");
    // 捕获一个引用，长度为 8
    let c3 = || println!("hello: {}", name);
    // 捕获移动的数据 name1（长度 24） + table（长度 48），closure 长度 72
    let c4 = move || println!("hello:{}, {:?}", name1, table);
    // 如果不 move，closure 大小是 16
    // ** 不带 move 时，闭包捕获的是变量的引用
    // ** 带 move 时，对应自由变量的所有权会被转移到闭包中
    //let c4 = || println!("hello:{}, {:?}", name1, table);

    let name2 = name.clone();
    // 和局部变量无关，捕获了一个 String name2，closure 长度 24
    let c5 = move || {
        let x = 1;
        let name3 = String::from("lindsey");
        println!("hello: {}, {:?}, {:?}", x, name2, name3);
    };

    println!(
        "c1: {}, c2:{}, c3:{}, c4:{}, c5:{}, main:{}",
        size_of_val(&c1),
        size_of_val(&c2),
        size_of_val(&c3),
        size_of_val(&c4),
        size_of_val(&c5),
        size_of_val(&main),
    );
}