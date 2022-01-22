# ENUM
## 1.Defining an Enum
```rust
enum IdAddr {
    V4,
    V6
}
```

Declare enum with field.
```rust
enum IdAddr {
    V4(String),
    V6(String)
}
```

Each variant can have different types and amounts of associated data.
```rust
enum IdAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}
```

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
This enum has four variants with different types:
- `Quit` has no data associated with it at all.
- `Move` has named fields like a struct does.
- `Write` includes a single `String`.
- `ChangeColor` includes three `i32` values.

Defining an enum with variant such as the ones in above is similar to defining different kinds of struct definitions, except the enum doesn't use the `struct` keyword and all the variants are grouped together under the `Message` type.

## NULL and Option
Rust use enum types to represent NULL. This enum is `Option<T>`, and it is defined by the standard library. The `Option` don't need to bring it into scope expilicitly. You can use `Some` and `None` directly without the `Option::` prefix.

```rust
enum Option<T> {
    None,
    Some(T),
}
```

```rust
let some_number = Some(5);
let some_string = Some("a String");
let absent_number: Option<i32> = None;
```