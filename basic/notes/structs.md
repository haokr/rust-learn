# Structs

定义 Structs：

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

实例化：

```rust
fn init_user() {
     let user1 = User {
         email: String::from("abc@abc.com"),
         username: "wanghao".to_string(),
         active: true,
         sign_in_count: 1,
     }
}
```

如果需要修改 `u1` ，需要将 `u1` 声明为可修改类型：`let mut u1 = User{}`。

需注意 `username` 的字段类型是 `String`，如果直接写字面量 `wanghao` 是错误的，因为字面量的类型是 `&str`，因此需要使用 `to_string()` 方法将字面量转为 `String` 类型。

如果我们这里将 `username` 指定为 `&str` 类型也是错误的，因为指定为 `&str` 类型需要指定生命周期，关于生命周期的概念将在后面介绍。

修改其中某一个属性：

```rust
u1.email = String::from("h@h.com");
```

如果使用参数创建实例时，如果参数名和属性名相同，可以使用简略写法：

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

使用属性：

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

在上面的代码中，`..user1` 语法直接将 `user1` 中除 `emil` 和 `username` 的属性填入到 `user2` 中。

## Unnamed fields（Tuple Structs）

```rust
struct Colort(i32, i32, i32);

let black = Color(0,0,0);
```

## Example

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

## Print

当我们尝试直接打印 structs 类型时：

```rust
let rect1 = Rectangle {
    width: 30,
    height: 50,
};

println!("rect1 is {}", rect1);
```

编译器会报错：`error[E0277]: Rectangle doesn't implement std::fmt::Display`，告诉我们 `Rectangle` 类型没有实现 `Display`。但我们可以通过实现 `Debug` 来让编译器使用默认的打印格式：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

首先在 `Rectangle` 声明时需要加一行 `#[derive(Debug)]`，然后在 `println` 语句中使用 `{:?}`，`?` 标识，表示使用 Debug 模式。

## Defining Methods

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

## Associated Function

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    Rectangle::square(10);
}
```