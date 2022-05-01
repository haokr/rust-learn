**多态：**
1. 动态类型系统，通过鸭子类型实现
2. 静态类型系统通过
    - 参数多态（paramertic polymorphism），指代码操作的类型是一个满足某些约束的参数，而非具体的类型
    - 特设多态（adhoc polymorphism），指同一种行为有多个不同实现的多态，比如加法，可以 1+1，也可以 abc + def
    - 子类型多态（subtype polymorphism），指在运行时，子类型可以被当成父类型使用

Rust 中参数多态使用泛型来支持，特设多态通过 trait 支持，子类型多态使用 trait object 来支持。

**类型系统基本概念：**  
![类型体统基本概念](https://static001.geekbang.org/resource/image/09/15/09ea90a4df9fb7652389f611412c1715.jpg?wh=3175x1490)

Rust 是强类型语言，且静态类型检查。

从内存角度看，类型安全是指代码，只能按照被允许的方法，访问它被授予访问权限的内存。

C/C++ 定义后数据可以隐式转换，是弱类型语言，不是内存安全的。而像 Rust 这样的强类型语言，是类型安全的。

Rust 中除了 let/fn/static/const 这些定义性的语句外，都是表达式，像
```rust
if has_work {
    do_something();
}
```
这样的，返回值是 unit，它的值和类型都是 `()`.

在 Rust 中对于一个作用域，无论是 if/for 还是番薯，最后一个表达式的返回值就是作用域的返回值，如果表达式不返回任何值，那么它返回一个 `unit()`.

**类型推导：**

有些情况下，即使上下文中含有类型的信息，也需要开发者为变量提供类型，比如常量和静态变量的定义。
```rust
const PI: f64 = 3.1415926;
static E: f32 = 2.71828;

fn main() {
    const V: u32 = 10;
    static V1: &str = "hello";
    println!("PI: {}, E: {}, V: {}, V1: {}", PI, E, V, V1);
}
```

**泛型数据结构**  

```rust
enum Option<T> {
    Some(T),
    None,
}
```

```rust
pub struct Vec<T, A: Allocator = Global> {
    buf: RawVec<T, A>,
    len: usize,
}

pub struct RawVec<T, A: Allocator = Global> {
    ptr: Unique<T>,
    cap: usize,
    alloc: A,
}
```

生命周期标注也是泛型的一部分。
```rust
pub enum Cow<'a, B: ?Sized + 'a> where B : ToOwned,
{
    // 借用的数据
    Borrowed(&'a B),
    // 拥有的数据
    Owned(<B as ToOwned>::Owned),
}
```

也可以在不同的实现下逐步添加约束。

**泛型函数**

泛型函数在编译时会单态化，这使代码执行效率变高，同时也使二进制文件变大，编译速度变慢。同时，编译后的代码会丢弃泛型信息。因此，分发带有泛型的代码时，不能以二进制文件形式分发，需要分发源码。

**Trait Object**
![Trait Object 实现机理](https://static001.geekbang.org/resource/image/49/1d/4900097edab0yye11233e14ef857be1d.jpg?wh=2248x1370)

HtmlFormatter 的引用赋值给 Formatter 后，会生成一个 Trait Object，在上图中可以看到，Trait Object 的底层逻辑就是胖指针。其中，一个指针指向数据本身，另一个则指向虚函数表（vtable）。

vtable 是一张静态的表，Rust 在编译时会为使用了 trait object 的类型的 trait 实现生成一张表，放在可执行文件中（一般在 TEXT 或 RODATA 段）。

![vtable](https://static001.geekbang.org/resource/image/9d/5e/9ddeafee9740e891f6bf9c1584e6905e.jpg?wh=2389x1738)

所以 Rust 里的 Trait Object 是 C++/Java 中的 vtable 的一个变体。

但是，只有满足对象安全的 trait 才能使用 trait object。[文档](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)

如果 trait 所有的方法，返回值是 Self 或带有泛型参数，那么这个 trait 就不能产生 trait object。

不允许返回 Self，是因为 trait object 在产生时，原来的类型会被抹去，所以 Self 究竟是谁不知道。比如 Clone trait 只有一个方法 clone()，返回 Self，所以它就不能产生 trait object。

不允许携带泛型参数，是因为 Rust 里带泛型的类型在编译时会做单态化，而 trait object 是运行时的产物，两者不能兼容。

疑问：trait object 不是根据具体的类型生成的吗？trait 泛型和 Self 经过单态化，在生成时不是可以根据具体类型得到吗？，为什么不能确定呢？