当进行变量赋值、传参和函数返回时，如果数据结构没有实现 Copy trait，就会默认使用 Move 语义转移值的所有权。

当不希望值的所有权被转移，又因为没有实现 Copy trait 而无法使用 Copy 语义时，可以使用 Borrow 语义。

Borrow 只是借出了临时使用权，通过引用语法 `&` 或者 `&mut` 来实现。

在 Rust 中借用和引用是一个概念。

在其他语言中，引用是一种别名，多个引用拥有对值的无差别的访问权限，本质上是共享了所有权，在 Rust 中，所有的引用都只是借用了临时的使用权，并不破坏值的单一所有权约束。

**Rust 所有的参数传递都是值传递。** 不管是 Copy 还是 Move，在 Rust 中必须显式地把某个数据引用传递给另一个函数。

Rust 的引用实现了 Copy trait，所以按照 Copy 语义，这个引用会被复制一份交给要调用的函数。对这个函数来说，它不拥有数据本身，数据只是临时借给他使用，所有权还在原来的函数那里。

**可变引用：**  
所以为了保证内存安全，Rust 对可变引用的使用也做了严格的约束：  
1. 在一个作用域内，仅允许一个活跃的可变引用。所谓活跃，就是真正被使用来修改数据的可变引用，如果只是定义了，却没有使用或者当作只读引用使用，不算活跃。  
2. 在一个作用域内，活跃的可变引用（写）和只读引用（读）是互斥的，不能同时存在。
 
应该是 immutable -> mutable -> immutable 这样的操作顺序不被允许，把 mutable 放在最前或最后都是可以的。

参考 `rustc --explain E0502`:  
A variable already borrowed as immutable was borrowed as mutable.  
Erroneous code example:

```
fn bar(x: &mut i32) {}
fn foo(a: &mut i32) {
    let y = &a; // a is borrowed as immutable.
    bar(a); // error: cannot borrow `*a` as mutable because `a` is also borrowed
            //        as immutable
    println!("{}", y);
}
```

To fix this error, ensure that you don't have any other references to the
variable before trying to access it mutably:

```
fn bar(x: &mut i32) {}
fn foo(a: &mut i32) {
    bar(a);
    let y = &a; // ok!
    println!("{}", y);
}
```

For more information on Rust's ownership system, take a look at the [References & Borrowing][references-and-borrowing] section of the Book.

[references-and-borrowing]: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

---

The Rules of References  

Let’s recap what we’ve discussed about references:

- At any given time, you can have either one mutable reference or any number of immutable   references.
- References must always be valid. 

---

使用数据结构时，数据结构自身的生命周期要小于等于其内部字段的所有引用的生命周期。