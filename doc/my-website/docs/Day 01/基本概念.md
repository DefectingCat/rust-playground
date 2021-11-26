## 变量与可变性

在 Rust 中变量默认是不可改变的（immutable）。如果需要改变一个变量的值，则需要显式的将其声明为可变变量：

```rust
fn main() {
    let mut m = 42;
    m = m + 1;
    println!("M is: {}", m);
}
```

### 不可变变量与常量的区别

常量（constants）。类似于不可变变量，常量是绑定到一个名称的不允许改变的值，不过常量与变量还是有一些区别。

* 常量总是不能变；
* 常量使用 `const` 关键字，而变量使用 `let` 声明；
* 常量**必须**注明值的类型；
* 常量可以在任何作用域中声明，包括全局作用域；
* 常量只能被设置为常量表达式，而不可以是其他任何只能在运行时计算出的值。

```rust
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
}
```

## 隐藏（Shadowing）

Rust 可以使用 `let` 关键字重复声明一个同名变量，这与将变量标记为 `mut` 是有区别的。通过使用 let，我们可以用这个值进行一些计算，不过计算完之后变量仍然是不可变的。

mut 与隐藏的另一个区别是，当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，但复用这个名字。

```rust
fn main() {
    let spaces = "                ";
    let spaces = spaces.len();

    println!("Spaces length: {}", spaces);
}
```