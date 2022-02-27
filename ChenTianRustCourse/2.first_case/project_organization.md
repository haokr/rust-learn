# Project Organization

当需要多个文件协同工作时，可以使用 `mod` 来组织代码。

使用 `mod module_name` 来引入同目录下的其他代码文件。注意，不能自己建 `lib.rs` 文件，这个命令空间是已存在的。

存在子目录时，不能直接通过 `mod` 关键字引入子目录下的文件。必须先在子目录下新建 `mod.rs` 文件，将需要暴露的文件通过 `pub mod file_name` 的方式暴露出去。然后，再在外部文件中使用 `mod sub_directory_name` 引入子目录，通过子目录的命名空间来访问子目录下的文件。

如果需要多个 `crates` 来组织代码，可以使用 `workspace`。一个 `workspace` 包含一到多个 `crates`。

新建 `workspace` 时，再要目录下生成一个 `Catgo.toml`，包含 `workspace` 里所有的 `crates`。

```toml
[workspace]

members = [
    "core",
    "server",
    "client",
]
```