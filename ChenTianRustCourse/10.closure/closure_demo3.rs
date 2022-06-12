fn main() {
    let name = String::from("tyr");

    // 捕获了变量 name
    let c = move |greeting: String| (greeting, name.clone());

    println!("c1 call once: {:?}", c("josh".into()));
    println!("c1 call twice: {:?}", c("divid".into()));

    // 被当作 FnOnce 调用 
    println!("result: {:?}", call_once("hi".into(), c));
    // 无法再次调用
    // let result = c("hi".to_string());
    
    // 没有捕获自由变量，可以多次调用
    let c2 = move |greeting: String| (greeting, "c2".into());
    println!("result: {:?}", call_once("hi once".into(), c2));
    println!("result: {:?}", call_once("hi tiwce".into(), c2));

    // Fn 当作 FnOnce 调用
    println!("result: {:?}", call_once("hola".into(), not_closure));
    // 可以再次调用，因为 fn 不是一个闭包，没有捕获自由变量
    println!("result: {:?}", call_once("hola".into(), not_closure));

}

fn call_once(arg: String, c: impl FnOnce(String) -> (String, String)) -> (String, String) {
    // FnOnce 的 call_once 传入了 self，因此只能调用一次
    c(arg)
} 

fn not_closure(arg: String) -> (String, String) {
    (arg, "Roise".into())
}