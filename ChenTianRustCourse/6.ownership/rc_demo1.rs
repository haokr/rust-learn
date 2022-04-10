fn main() {
    let arr = vec![1];

    // use move to give ownership to thread
    let t = std::thread::spawn(move || {
        println!("{:?}", arr)
    });
    // join 一下等待线程执行完成
    t.join().unwrap();
}