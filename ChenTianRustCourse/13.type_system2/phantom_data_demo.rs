
use std::marker::PhantomData;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Identifier<T> {
    inner: u64,
    // rust 不允许未使用的泛型，因此使用 PhantomData 来持有暂时不需要的泛型
    // PhantomData 实际长度为 0，唯一的作用时类型标记
    _tag: PhantomData<T>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct User {
    id: Identifier<Self>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Product {
    id: Identifier<Self>,
}

fn main() {
    let user = User::default();
    let user2 = User::default();

    let product = Product::default();

    // 两个 id 不能比较，因为他们属于不同的类型
    // assert_ne!(user.id, product.id);
    assert_eq!(user.id, user2.id);

    assert_eq!(user.id.inner, product.id.inner);
}