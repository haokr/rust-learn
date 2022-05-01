use std::ops::Add;

#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    pub fn new(real: f64, imagine: f64) -> Self {
        Self { real, imagine }
    }
}

/// 泛型 trait
impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Self::new(real, imagine)
    }
}

// 为引用类型实现 trait
impl Add for &Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Complex::new(real, imagine)
    }
}

impl Add<&Complex> for f64 {
    type Output = Complex;

    fn add(self, rhs: &Complex) -> Self::Output {
        let real = rhs.real + self;
        Complex::new(real, rhs.imagine)
    }
}

fn main() {
    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2 as f64, 3.0);
    println!("{:?}", &c1 + &c2);
    // 报错，Complex 没有实现 Add trait
    // println!("{:?}", &c1 + 6.0);
    // 正常运行，可以看出，是调用第一个参数（6.0）的 add 方法
    println!("{:?}", 6.0 + &c1);
    println!("{:?}", c1 + c2);
}

