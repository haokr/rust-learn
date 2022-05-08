use std::{fmt, ops::Deref, str};

const MINI_STRING_MAX_LEN: usize = 30;

// MyString 里，String 有 3 个 word，供 24 字节，所以它以 8 字节对齐
// 所以 enum 的 tag + padding 最少 8 字节，整个结构占 32 字节
// MiniString 可以最多有 30 字节（加上 padding、tag 共 32 字节）
struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    // 隐藏 new
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        let len = bytes.len();

        let mut data = [0u8; MINI_STRING_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        // 由于 MiniString new 方法是隐藏的，它只能来自字符串，所以下面这行是安全的
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
        // unsafe 版本
        // unsafe { str::from_utf8_unchecked(&self.data[..self.len as usize]) }
    }
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 实现了 deref
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            MyString::Inline(ref v) => v.deref(),
            MyString::Standard(ref v) => v.deref(),
        }
    }
}

impl<T> From<T> for MyString
where T: AsRef<str> + Into<String>,
{
    fn from(s: T) -> Self {
        match s.as_ref().len() > MINI_STRING_MAX_LEN {
            true => Self::Standard(s.into()),
            _ => Self::Inline(MiniString::new(s)),
        }
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl MyString {
    fn push_str(&mut self, string: &str) {
        match self {
            MyString::Inline(m) => {
                let l = m.len();
                let len = l + string.len();
                if len > MINI_STRING_MAX_LEN {
                    *self = Self::Standard(m.to_string() + string);
                } else {
                    m.data[l..len].copy_from_slice(string.as_bytes());
                    m.len = len as u8;
                }
            }
            MyString::Standard(s) => s.push_str(string),
        }
    }
}

fn main() {
    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    println!("Len: MyString {}, MiniString: {}", len1, len2);

    let s1:MyString = "这没超过三十个字符。".into();
    let s2:MyString = "这是一个超过三十个字符的字符传。".into();

    println!("s1: {:?}, s2: {:?}", s1, s2);

    println!(
        "s1: {}({} bytes, {} chars);\ns2: {}({} bytes, {} chars)",
        s1, s1.len(), s1.chars().count(),
        s2, s2.len(), s2.chars().count()
    );

    // assert!(s1.ends_with("world!"));
    // assert!(s2.starts_with("这"));

    let mut s3: MyString = "OKOK".to_string().into();
    println!("s3: {:?}", s3);

    s3.push_str(", fine.");
    println!("s3: {:?}", s3);
}