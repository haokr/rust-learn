use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct Buffer<T>(Vec<T>);

impl<T> Buffer<T> {
    pub fn new(v: impl Into<Vec<T>>) -> Self {
        Self(v.into())
    }
}

/// 实现解引用，直接通过 buf 访问 buf.0
impl<T> Deref for Buffer<T> {

    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Buffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut buf = Buffer::new([1,2,4,3]);
    /// 相当于 buf.0.sort()
    /// 这是因为 sort() 方法的第一个参数是 &mut self，此时 Rust 编译器会强制做 Deref/DerefMut 的解引用
    /// 相当于 (*(&mut buf)).sort()
    buf.sort();
    println!("buf: {:?}", buf);
}