`if let` 语法让我们以一种不那么冗长的方式结合 `if` 和 `let`，来处理只匹配一个模式的值而忽略其他模式的情况。

如果我们想要对 `Some(3)` 匹配进行操作但是不想处理任何其他 `Some<u8>` 值或 `None` 值。为了满足 `match` 表达式（穷尽性）的要求，必须在处理完这唯一的成员后加上 `_ => ()`，这样也要增加很多样板代码。

```rust
fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
}
```

我们可以使用 `if let` 这种更短的方式编写。使用 `if let` 意味着编写更少代码，更少的缩进和更少的样板代码。然而，这样会失去 `match` 强制要求的穷尽性检查。`match` 和 `if let` 之间的选择依赖特定的环境以及增加简洁度和失去穷尽性检查的权衡取舍。

换句话说，可以认为 `if let` 是 `match` 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。

```rust
fn main() {
    let some_u8_value = Some(3);

    if let Some(3) = some_u8_value {
        println!("three")
    }
}
```

这有点类似于那个语言中 `if` 语句自动转换为布尔值的感觉：

```js
const someValue = 3;
if (someValue == 3) {
  console.log("three");
}
```

## 小结

`Option<T>` 类型帮助我们利用类型系统来避免出错的。当枚举值包含数据时，可以根据需要处理多少情况来选择使用 `match` 或 `if let` 来获取并使用这些值。

我们的 Rust 程序现在能够使用结构体和枚举在自己的作用域内表现其内容了。在我们的 API 中使用自定义类型保证了类型安全：编译器会确保我们的函数只会得到它期望的类型的值。