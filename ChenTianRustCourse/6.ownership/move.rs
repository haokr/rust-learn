fn main() {
    let data = vec![2,3,4];
    let data1 = &data;

    // 值的地址和引用的地址
    println!("addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}", 
        &data, data1, &&data, &data1
    );

    println!("sum of data1: {}", sum(data1));

    // 堆上数据的地址，{:p} 打印指针地址
    println!("addr of items: [{:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2]
    );
}

fn sum(data: &Vec<u32>) -> u32 {
    // 值的地址不会变，引用地址会变
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}