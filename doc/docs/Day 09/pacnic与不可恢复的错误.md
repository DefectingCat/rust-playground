Rust 将错误组合成两个主要类别：**可恢复错误**（recoverable）和 **不可恢复错误**（unrecoverable）。可恢复错误通常代表向用户报告错误和重试操作是合理的情况，比如未找到文件。不可恢复错误通常是 bug 的同义词，比如尝试访问超过数组结尾的位置。

大部分语言并不区分这两类错误，并采用类似异常这样方式统一处理他们。Rust 并没有异常，但是，有可恢复错误 `Result<T, E>` ，和不可恢复(遇到错误时停止程序执行)错误 `panic!`。

## 对应-panic-时的栈展开或终止

当出现 panic 时，程序默认会开始 **展开**（unwinding），这意味着 Rust 会回溯并清理数据。另一种选择是直接 **终止**（abor），这会将程序使用的内存交给操作系统来清理。panic 时通过在 Cargo.toml 的 `[profile]` 部分增加 `panic = 'abort'`，可以由展开切换为终止。

```tomal
[profile.release]
panic = 'abort'
```

一个非常简单的程序大概的大小区别（Ubuntu 20.04 lts）

```bash
# unwinding
-rwxr-xr-x 2 root root 3.6M Dec  6 02:06 fibonacci_test
# abort
-rwxr-xr-x 2 root root 3.5M Dec  6 02:06 fibonacci_tes
```

可以使用 panic 宏直接发起一个 panic：

```rust
fn main() {
    panic!("crash and burn");
}
```

## backtrace

panic 时可以通过设置 `RUST_BACKTRACE` 环境变量来得到一个 backtrace，backtrace 就是类似函数调用栈，它会列出所有执行到目前位置所有被调用的函数列表。

```bash
RUST_BACKTRACE=1 cargo run
```

