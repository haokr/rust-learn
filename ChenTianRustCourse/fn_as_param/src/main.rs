fn main() {
    println!("apply square: {}", apply(11, square));
}


fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}