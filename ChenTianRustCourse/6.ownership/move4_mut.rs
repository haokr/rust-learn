fn main() {
    let mut data = vec![1,2,3];
    // immutavle borrow
    let data1 = vec![&data[0]];
    println!("data[0]: {:p}", &data[0]);

    // mutable borrow
    for i in 0..100 {
        data.push(i);
    }

    // 如果继续添加元素，堆上的数据预留的空间不够了，就会重新分配一片足够大的内存，
    // 把之前的值拷过来，然后释放旧的内存。这样就会让 data1 中保存的 &data[0] 
    // 引用失效，导致内存安全问题。
    // rustc --explain E0502

    // immutable borrow
    println!("data[0]: {:p}", &data[0]);
    println!("boxed: {:p}", &data1);
}