// ERROR
// fn main() { 
//     let mut arr = vec![1, 2, 3]; 
//     // cache the last item 
//     let last = arr.last(); 
//     arr.push(4); 
//     // consume previously stored last item 
//     println!("last: {:?}", last);
// }

// FIX
fn main() {
    let mut arr = vec![1, 2, 3];
    // 这样写为什么可行？
    // 用下标取值，等价于 let a = u32;
    // u32 实现了 Copy trait，所以 last 拿到的实际上是新 copy 的
    // 而 last() 方法返回的是 Option 包裹的 arr 中的值，所以不可行
    let last = arr[arr.len()-1];
    arr.push(4);
    println!("last: {:?}", last);
}