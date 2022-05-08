fn main() {
    let a = -7;
    let b = 3;
    /// trunc 向 0 取整
    /// -7 % 3 = -7 - trunc(-7/3) * 3 = -7 - (-2) * 3 = -7 + 6 = -1
    println!("{}", &a % &b);
    println!("{}", &a / &b);

}