fn main() {
    let numbers = vec![1,2,3,4,5,6];

    // 报错，因为 rust 无法从 iter 推导出具体的类型
    // let event_numbers = numbers
    //         .into_iter()
    //         .filter(|n| n % 2 == 0)
    //         .collect();

    // let event_numbers = numbers
    //         .into_iter()
    //         .filter(|n| n % 2 == 0)
               // 在泛型后使用 ::<T> 来强制使用类型 T，这种写法被称为 turbofish
    //         // 指定 Vec 类型，具体元素类型 rust 可以推导出，因此使用 _ 简写
    //         .collect::<Vec<_>>();
            
    // 指定 Vec 类型，具体元素类型 rust 可以推导出，因此使用 _ 简写
    let event_numbers: Vec<_> = numbers
            .into_iter()
            .filter(|n| n % 2 == 0)
            .collect();

    println!("{:?}", event_numbers);
}