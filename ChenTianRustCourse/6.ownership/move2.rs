fn main() {
    let r = local_ref();
    println!("r: {:p}", r);
}

fn local_ref<'a>() -> &'a i32 {
    let a = 42;
    // a 的引用已经被回收，不可以返回 a 的引用
    &a
}