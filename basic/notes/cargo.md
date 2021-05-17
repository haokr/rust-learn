# 使用 Cargo 管理 rust 项目

> [Hello, Cargo]([Hello, Cargo! - The Rust Programming Language (rust-lang.org)](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html))

## 1. Create a new project

Rust 使用 cargo 作为 Rust 的包管理。安装 rust 时会默认安装 cargo，无需额外安装。

```bash
cargo --version
```

使用 cargo 创建项目

```bash
cargo new project_name
```

执行 `new` 命令之后会生成一个以 project_name 为名的目录，里面包含一个配置文件 *Cargo.toml* 和一个代码目录 `src`。

此时 *Cargo.toml* 文件内包含如下内容：

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["wanghao"] # cargo 自动从环境中获取，如果指定参数 --vcs=git，则会展示如下格式
# authors = ["Your Name <you@example.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

另外，在执行 `cargo new` 命令时可以指定参数 `--vcs`，如：`cargo new --vcs=git`，此时会自动生成 *.gitignore* 文件。

> 关于 TOML（Tom's Obvious, Minimal Language）的更多信息：[TOML]([TOML: Tom's Obvious Minimal Language](https://toml.io/en/))

最后一行是 `[dependencies]`，在这个条目下写的是依赖包，在 Rust 中，依赖包被叫做 `crates`。

## 2. Build and run

编译 cargo 项目只需要执行命令 `cargo build`。

编译生成的可执行文件在`./target/debug/hello_word`。

在第一次执行 `cargo build` 时，cargo 会在项目根目录下生成一个 *Cargo.lock* 文件，这个文件的作用是跟踪项目中使用的依赖包版本。

> Running `cargo build` for the first time also causes Cargo to create a new file at the top level: *Cargo.lock*. This file keeps track of the exact versions of dependencies in your project. 

运行项目时，可以直接 `./` 执行 `./target/debug/hello_word` 也可以使用命令 `cargo run` 运行项目。

当只想检测代码是否有问题时，cargo 提供了 `cargo check` 命令，该命令会编译代码，但不会生成可执行文件，执行 `cargo check` 的速度要比 `cargo build` 快的多。

当想要发布时执行 `cargo build --release` 该命令会在 `/target/release` 目录下生成可执行文件。并且会在编译时进行优化。

