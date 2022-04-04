fn main() {
    let mut x = Vec::new();
    // 其实这样写是可以的，因为不可变引用 y 在 x.push 之后没有使用过，编译器做了优化
    // 当在 x.push 之后再使用 y 会报错
    // 另外，按照原本的写法 let y = x[0]; 即使在 x.push 之后再使用 y 也不会报错
    // 因为这里的 y 是新 copy 的一份，并不是 vec 堆中的那个数据
    {
        let ptr = &mut x; // Take a mutable reference to `x`
        ptr.push(1); // Allowed
        let y = x.first(); // Not allowed (will not compile): as long as `ptr` is active,
                    // x cannot be read from ...
        x.push(1);    // .. or written to
    }
    println!("x:{:?}", x);

    // alternatively,

    let mut x = Vec::new();
    x.push(1); // Allowed
    {
        let ptr = &x; // Create an immutable reference
        let y = ptr[0]; // Allowed, nobody can mutate
        let y = x[0]; // Similarly allowed
        x.push(1); // Not allowed (will not compile): as long as `ptr` is active,
                // `x` is frozen for mutation
    }
    println!("x:{:?}", x);

}