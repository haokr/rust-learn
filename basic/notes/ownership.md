# Ownership

> [What is ownership]([What is Ownership? - The Rust Programming Language (rust-lang.org)](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html))

> Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses. 

## The ownership rules

- Each value in Rust has a variable that’s called its *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Example

```rust
fn variable_scope() {
    let s = "hello";
} // the scopre is over and  the variable s is unvalid, the memery of s will be recycled
```

当方法结束时，`variable_scope` 方法的执行域结束，方法中的变量也会进行销毁，回收对应的内存，因此变量 `s` 会被回收。