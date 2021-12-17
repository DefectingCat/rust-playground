之前的 I/O 项目首先将参数收集为一个 `Vec<String>`，然后使用 `clone` 得到其值。起初这里需要 `clone` 的原因是参数 `args` 中有一个 `String` 元素的 slice，而 `new` 函数并不拥有 `args`。为了能够返回 `Config` 实例的所有权，我们需要克隆 `Config` 中字段 `query` 和 `filename` 的值，这样 `Config` 实例就能拥有这些值。

在学习了迭代器之后，我们可以将 `new` 函数改为获取一个有所有权的迭代器作为参数而不是借用 slice。我们将使用迭代器功能之前检查 slice 长度和索引特定位置的代码。这会明确 `Config::new` 的工作因为迭代器会负责访问这些值。

一旦 `Config::new` 获取了迭代器的所有权并不再使用借用的索引操作，就可以将迭代器中的 `String` 值移动到 `Config` 中，而不是调用 `clone` 分配新的空间。

## 直接使用 env::args 返回的迭代器

`env::args` 函数返回一个迭代器！不同于将迭代器的值收集到一个 vector 中接着传递一个 slice 给 `Config::new`，现在我们直接将 `env::args` 返回的迭代器的所有权传递给 `Config::new`。

```rust
pub fn new(mut args: std::env::Args) -> Result<Self, &'static str> {
```

`env::args` 函数的标准库文档显示，它返回的迭代器的类型为 `std::env::Args`。我们更新了其签名，使其得到 `args` 的迭代器，因为我们拥有 `args` 的所有权。

## 使用 Iterator trait 代替索引

接下来，我们将修改 `Config::new` 的内容。标准库文档还提到 `std::env::Args` 实现了 `Iterator` trait，因此我们知道可以对其调用 `next` 方法！

```rust
pub fn new(mut args: std::env::Args) -> Result<Self, &'static str> {
    args.next(); // Program path

    Ok(Self {
        query: match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        },
        filename: match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        },
        case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
    })
}
```

`env::args` 返回值的第一个值是程序的名称。我们希望忽略它并获取下一个值，所以首先调用 `next` 并不对返回值做任何操作。之后对希望放入 `Config` 中字段 query 调用 `next`。如果 `next` 返回 `Some`，使用 `match` 来提取其值。如果它返回 `None`，则意味着没有提供足够的参数并通过 `Err` 值提早返回。对 `filename` 值进行同样的操作。

## 使用迭代器适配器来使代码更简明

可以通过使用迭代器适配器方法来编写更简明的代码。这也避免了一个可变的中间 `results` vector 的使用。函数式编程风格倾向于最小化可变状态的数量来使代码更简洁。去掉可变状态可能会使得将来进行并行搜索的增强变得更容易，因为我们不必管理 `results` vector 的并发访问。

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

同样的，我们也可以对 `search_case_insensitive` 和 `run` 方法进行同样的改动。

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    if config.case_sensitive {
        print_result(search(&config.query, &content));
    } else {
        print_result(search_case_insensitive(&config.query, &content))
    };

    Ok(())
}

pub fn print_result(result: Vec<&str>) {
    result.iter().map(|line| println!("{}", line)).collect()
}
```
