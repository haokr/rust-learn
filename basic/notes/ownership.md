# Ownership

> [What is ownership]([What is Ownership? - The Rust Programming Language (rust-lang.org)](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html))

> Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses. 

## The ownership rules

- Each value in Rust has a variable that’s called its *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
- At any given time, you can have *either* one mutable reference *or* any number of immutable references.
- References must always be valid.

```rust
fn variable_scope() {
    let s = "hello";
} // the scopre is over and  the variable s is unvalid, the memery of s will be recycled
```

当方法结束时，`variable_scope` 方法的执行域结束，方法中的变量也会进行销毁，回收对应的内存，因此变量 `s` 会被回收。

进行回收时，Rust 会自动调用 `drop` 方法。

## Move data

```rust
let x = 1;
let y = x;
```

上面这个例子中，`x` 和 `y` 都是简单类型的变量，直接存在栈内存中，因此 `let y = x;` 的赋值操作会复制一份  `1` 赋值给 `y`。

那存在堆内存中的复杂类型是如何操作的呢？看下面这个例子：

```rust
let s1 = String::from("hello");
let s2 = s1;
```

我们知道，ownership 会在方法结束时自动调用 `drop` 方法，回收方法内使用的变量，但是当 `let s2 = s1;` 操作后，会有两个指针指向同一块内存区域，因此当进行内存回收时，会导致重复内存回收，着显然是不可以的。

因此 Rust 对于复杂类型在进行诸如 `let s2 = s1;` 这样的赋值操作时，会销毁 `s1` 的指针，由 `s2` 指向该对象的堆内存地址。

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

因此，当执行如上操作时，会在编译时产生报错：

```bash
$ cargo build
   Compiling ownership v0.1.0 (/mnt/e/repos/rust-learn/basic/ownership)
error[E0382]: borrow of moved value: `s1`
  --> src/main.rs:22:20
   |
20 |     let s1 = String::from("hello");
   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
21 |     let s2 = s1;
   |              -- value moved here
22 |     println!("{}", s1);
   |                    ^^ value borrowed here after move

error: aborting due to previous error; 2 warnings emitted

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership`

To learn more, run the command again with --verbose.
```

另外，当字符串赋值形式如下时：

```rust
let s1 = "hello";
let s2 = s1;

println!("s1 = {}, s2 = {}.", s1 ,s2);
```

此时执行编译执行代码不会报错：

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `/mnt/e/repos/rust-learn/basic/ownership/target/debug/ownership`
s1 = hello, s2 = hello.
```

这是因为通过 `let s1 = "hello";` 这种方式创建的字符串直接保存在编译后的二进制文件中，并不保存在堆内存上，此时 `s1` 的数据类型是 `str`。

相反的，通过 `let s1 = String::from("hello");` 方式创建的字符串是 `String` 类型的，保存在堆内存上。

## Clone  and copy data

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}.", s1, s2);
```

当要执行**deep copy**操作时，需要调用 `clone` 方法。

当赋值的对象是保存在栈内存中的简单数据类型时，直接使用 `=` 即可：

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}.", x, y);
```

## Ownership and functions

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    println!("{}", s);				// 这一行会报错，因为此时 s 在 takes_ownership
    								// 函数中已经被回收了
    
    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    println!("{}", x);				// 这一行不会报错，因为 x 是 i32 类型，是简单类型
    								// 在作为参数传入函数时是直接复制的，因此 x 不会被回收
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```
有返回值的方法：


```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

当 `s1` 传入函数之后，在函数结束时，`s1` 就被回收了，因此在 `main` 方法中，调用方法之后 `s1` 就是 `borrowed` 状态，不可以使用。如果需要继续使用 `s1` 需要在函数中将 `s1` return 出来，写成 `let (s1, len) = func(s1);` 的形式。

在 ownership 中，最关键的点就是当方法执行完成之后会对方法内的变量调用 `drop` 方法回收内存空间，如果方法参数是保存在堆内存内的变量同样也会回收，因此会导致主方法内的变量（与方法参数指向同一块堆内存）也被回收掉。如果主方法中需要继续使用该变量，则需要在方法中将该参数 return 出来，并且在主方法中使用一个变量接收。

另外为避免重复回收的问题，当执行类似 `let s2 = s1;` 的操作时，如果 `s1` 时指向堆内存的变量，则会将 `s2` 指向 `s1` 指向的堆内存，并清除 `s1` 的指针，这样同样会导致 `s1` 变量不可用。

如果变量是诸如 `i32`等简单类型，在赋值和传参时会直接复制，因此不会存在上述问题。

## [Reference and borrowing]([References and Borrowing - The Rust Programming Language (rust-lang.org)](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html))

那有没有更好的写法呢？答案是有的，看下面的代码：

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String)-> usize {
    s.len()
}
```

上面段代码中，`calculate_length` 传入了一个**引用**（没有传入 ownership），因此执行过 `calcualte_length` 方法后，主方法中还拥有 `s1` 变量。

>The issue with the tuple code in Listing 4-5 is that we have to return the `String` to the calling function so we can still use the `String` after the call to `calculate_length`, because the `String` was moved into `calculate_length`.
>
>Here is how you would define and use a `calculate_length` function that has a reference to an object as a parameter instead of taking ownership of the value:
>
>First, notice that all the tuple code in the variable declaration and the function return value is gone. Second, note that we pass `&s1` into `calculate_length` and, in its definition, we take `&String` rather than `String`.

<img src="E:\repos\rust-learn\basic\notes\img\image-20210519184418031.png" alt="image-20210519184418031" style="zoom:80%;" />

相当于在 `calcuate_length` 方法中新声明了一个 变量 `s`，`s` 中的指针指向变量 `s1`（但不拥有 `s1`），因此当方法执行结束回收变量 `s` 不会对主方法中的变量 `s1` 造成影响。变量 `s` 是 `s1` 的一个引用。

传入变量引用的方式被成为参数借用。

> We call having references as function parameters *borrowing*. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back.

当然，使用参数借用的方式传入的参数，在方法内部不能对该参数进行修改。但可以通过为参数加上 `mut` 修饰符以允许方法对参数进行修改，并主方法中该变量也必须是可修改的（被 `mut` 修饰的）。

可变引用的一些限制：

> But mutable references have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope. This code will fail:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
```

上述代码会报错，因为 `s` 只能被借出一次，也就是说，同时只能存在一个 `s` 的可变引用。

形如：

```rust
fn reference_2() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;

    println!("{}, {}, {}", s, r1, r2);
}

fn reference_3() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
```

都是错误的，但是如下这种写法是可行的：

```rust
fn mut_reference() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
}
```

因为 `r1` 是放在一个花括号里面的，花括号会创建一个执行域，在这个域里面只存在一个 `s` 的可变引用，当花括号执行结束时，`r1` 会被回收，因此创建 `r2` 时不会产生错误。

形如下面的这种写法也是正确的：

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
```

**注意：如下写法是错误的：**

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); 

    let r3 = &mut s; // error
    println!("{}, {}, {}", r1, r2, r3);
}
```

官方解释是这样的：

> The scopes of the immutable references `r1` and `r2` end after the `println!` where they are last used, which is before the mutable reference `r3` is created. These scopes don’t overlap, so this code is allowed.

在上面的写法中，`r1`,` r2` 的使用范围与 `r3` 不重叠，因此是允许的，下面的写法中，`r1`，`r2`，`r3` 之间的使用范围有重叠，因此不允许。 

## Dangling reference

Rust 会在编译时确保没有空指针的错在，当声明一个指针时，Rust 会确认该指针指向的内存没有被回收，因此，如下形式的代码是错误的：

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

当 `dangle` 方法结束时，`s` 将会被回收，而我们尝试 return `s` 的一个引用，因此该引用实际上指向了一块无效的内存，Rust 不允许这样做。

## The slice type

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

<img src="E:\repos\rust-learn\basic\notes\img\image-20210519224120240.png" alt="image-20210519224120240" style="zoom:80%;" />

当切片包含第一个元素或最后一个元素时的简略写法：

```rust
#![allow(unused)]
fn main() {
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
}
```

```rust
#![allow(unused)]
fn main() {
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
}
```

```rust
#![allow(unused)]
fn main() {
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];
}
```

当需要获取 `String` 的片段时可以写出如下代码：

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn main() {
    let s = String::from("hello world");
    let s1 = first_word(&s);
    println!("{}", s1);
}
```

这里切片返回的类型是 `&str`。另外需注意的时，当对 `s` 进行切片之后将不允许再修改 `s`，因此，如下的代码是错误的：

```rust
fn main() {
    let mut s = String::from("hello world");
    let s1 = first_word(&s);
    println!("{}", s1);
    s.push_str("abc"); // error
    s.clear(); // error
    println!("{}", s1);
}
```

 