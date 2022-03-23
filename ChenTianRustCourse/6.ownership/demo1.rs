fn main() {
    let data = vec![1,2,3,4];
    let data1 = data;   // 此时 data1 指向了 vec 值，data 失效，后面的代码再使用 data 变量会报异常

    println!("sum of data1: {}", sum(data1));   // data1 被传入 sum 函数，vec 值被传给了 sum 函数的参数，当 sum 执行结束时，vec 值被回收
    println!("data1: {:?}", data1);     // error：经过上一行，data1 已不再持有 vec 值，并且 vec 值已被回收，这里会报错
    println!("sum of data: {}", sum(data));     // error：data 已不再持有 vec 值，这里会报错
}

fn sum(data: Vec<u32>) -> u32 {
    // 这里参数直接接受了 vec 值，当方法执行结束时，vec 值会被回收
    data.iter().fold(0, |acc, x| acc + x)
}