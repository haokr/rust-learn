fn main() {
    let name = String::from("Tyr");
    let c = move |greeting: String| (greeting, name);

    let result = c("hello".to_string());
    println!("result: {:?}", result);
    // 报错，name 的所有权已经被转移到闭包了
    // println!("name: {:?}", name);
}