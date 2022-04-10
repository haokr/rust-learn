/// 在一个作用域内，只允许一个活跃的可变引用。
/// 所谓活跃，就是真正被使用来修改数据的可变引用，
/// 如果只是定义了，却没有使用或者只当作只读引用，不算活跃。
/// 
/// 下面例子，如果调换 使用 v、vv、data 的顺序，则编译不通过
fn main() {
    let mut data = 1;
    // data 的可变借用 v
    let mut v = &mut data;
    // v 的可变借用 vv，此时只有 vv 是活跃的
    let vv = &mut v;
    **vv += 1;    
    // drop v 的可变借用 vv
    // 此时 vv 已经被 drop，只有 v 是活跃的
    *v += 1;
    // drop data 的可变借用 v
    println!("data: {:?}", data);
}