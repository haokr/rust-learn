enum StringOrInt {
    Str(String),
    Int(i64)
}

fn main() {
    let mut x = StringOrInt::Str("Hi".to_string());
    let y = &mut x;

    if let StringOrInt::Str(ref inside) = x {
        *y = StringOrInt::Int(1);
        println!("x says: {}", inside);
    }
}