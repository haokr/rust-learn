fn main() {
    println!("Hello, world!");
}

fn variable_scope() {
    let s = "hello";
} // the scopre is over and  the variable s is unvalid, the memery of s will be recycled


fn variable_in_heap() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // append a literal to a String

    println!("{}, s");
}
