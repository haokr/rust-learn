include!("reference.rs");
include!("slice.rs");

fn main() {
    let mut s = String::from("hello world");
    let s1 = first_word(&s);
    println!("{}", s1);
    s.push_str("abc");
    s.clear();
    println!("{}", s1);
}








fn main3() {
    let s1 = String::from("hello");
    let s1 = take_and_return(s1);
    println!("{}", s1);
}

fn take_and_return(s: String)-> String {
    s
}


fn main2() {
    let s = String::from("hello");

    takes_ownership(s);

    // println!("In main, s={}", s);

    let x = 5;

    makes_copy(x);

    println!("In main, s={}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}


fn variable_scope() {
    let s = "hello";
} // the scopre is over and  the variable s is unvalid, the memery of s will be recycled


fn variable_in_heap() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // append a literal to a String

    println!("{}", s);
}


fn two_variable_point_a_string() {
    let s1 = String::from("hello");
    let s2 = s1;
//    println!("{}", s1);
}

fn string_literal_move() {
    let s1 = "hello";
    let s2 = s1;

    println!("s1 = {}, s2 = {}.", s1, s2);
}


fn data_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}.", s1, s2);
}
