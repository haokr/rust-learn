/// RUNNABLE
fn main() { 
    let mut data: Vec<&u32> = Vec::new(); 
    let v = 42; 
    data.push(&v); 
    println!("data: {:?}", data);
}

/// ERROR
fn main1() {
    let mut data: Vec<&u32> = Vec::new();
    push_local_ref(&mut data);
    println!("data: {:?}", data);
}

fn push_local_ref(data: &mut Vec<&u32>) {
    let v = 42;
    // ERR：v 的引用被回收了
    data.push(&v);
}

/// 应该改成这样：
fn main2() {
    let mut data: Vec<u32> = Vec::new();
    push_local_ref(&mut data);
    println!("data: {:?}", data);
}

fn push_local_ref(data: &mut Vec<u32>) {
    let v = 42;
    // 虽然 v 的引用被回收了，但可以 return 数据 v 本身
    data.push(v);
}