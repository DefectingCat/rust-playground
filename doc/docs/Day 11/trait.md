trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共享的行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。

## 定义 trait

一个类型的行为由其可供调用的方法构成。如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了。trait 定义是**一种将方法签名组合起来的方法**。目的是定义一个实现某些目的所必需的行为集合。

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

这里使用 `trait` 关键字来声明一个 trait，后面是 trait 的名字。在大括号中声明描述实现这个 trait 的类型所需要的行为的方法签名，只需要方法签名即可，不需要方法具体实现。

trait 体中可以有多个方法：一行一个方法签名且都以分号结尾。

## 为类型实现 trait

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

在类型上实现 trait 类似于实现与 trait 无关的方法。区别在于 `impl` 关键字之后，我们提供需要实现 trait 的名称，接着是 `for` 和需要实现 trait 的类型的名称。在 `impl` 块中，使用 trait 定义中的方法签名，不过不再后跟分号，而是需要在大括号中编写函数体来为特定类型实现 trait 方法所拥有的行为。

一旦实现了 trait，我们就可以用与 `NewsArticle` 和 `Tweet` 实例的非 trait 方法一样的方式调用 trait 方法了：

```rust
let my_tweet = Tweet {
    username: String::from("xfy"),
    content: String::from("xfy! xfy!! xfy!!!"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", my_tweet.summarize());
```

实现 trait 时需要注意的一个限制是，只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait。例如，可以为 `aggregator` crate 的自定义类型 Tweet 实现如标准库中的 `Display` trait，这是因为 Tweet 类型位于 `aggregator` crate 本地的作用域中。类似地，也可以在 `aggregator` crate 中为 `Vec<T>` 实现 `Summary`，这是因为 `Summary` trait 位于 `aggregator` crate 本地作用域中。

但是不能为外部类型实现外部 trait。例如，不能在 `aggregator` crate 中为 `Vec<T>` 实现 `Display` trait。这是因为 `Display` 和 `Vec<T>` 都定义于标准库中，它们并不位于 `aggregator` crate 本地作用域中。这个限制是被称为 **相干性**（coherence） 的程序属性的一部分，或者更具体的说是 **孤儿规则**（orphan rule），其得名于不存在父类型。这条规则确保了其他人编写的代码不会破坏你代码，反之亦然。没有这条规则的话，两个 crate 可以分别对相同类型实现相同的 trait，而 Rust 将无从得知应该使用哪一个实现。

## 默认实现

有时为 trait 中的某些或全部方法提供默认的行为，而不是在每个类型的每个实现中都定义自己的行为是很有用的。这样当为某个特定类型实现 trait 时，可以选择保留或重载每个方法的默认行为。

也就是说直接在 trait 中声明整个函数体，之后当某个类型实现了这个 trait 后，变保留这个默认方法。

```rust
pub trait Summary {
    fn summarize(&self) -> String;
    fn read_more(&self) -> String {
        String::from("(Read more...)")
    }
}
```

如果仅仅只需要这个默认实现，则指定一个空的 `impl` 块 `impl Summary for NewsArticle {}`。

默认实现运行调用相同 trait 中的其他方法，哪怕这些方法没有默认实现。如此，trait 可以通过很多有用的功能而只需要实现指定一小部分内容。

```rust
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

例如这里的 Tweet 结构体只实现了 `summarize` 方法，无需重新声明对应的默认方法 `read_more`即可实现默认实现。

> 无法从相同方法的重载实现中调用默认方法。

## trait 作为参数

我们可以定义一个函数并将指定的 trait 作为参数来调用该 trait 上的方法。

```rust
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize())
}
```

对于 `item` 参数，我们指定了 `impl` 关键字和 trait 名称，而不是具体的类型。该参数支持任何实现了指定 trait 的类型。在 `notify` 函数中，可以调用任何来自 `Summary` trait 的方法。我们可以传递任何 `NewsArticle` 或 `Tweet` 的实例来调用 `notify`。任何用其它如 `String` 或 `i32` 的类型调用该函数的代码都不能编译，因为它们没有实现 `Summary`。

### Trait Bound 语法

`impl Trait` 语法用于直观的例子，它实际上是一种较长形式的语法糖。我们称之为 _trait bound_。

```rust
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize())
}
```

trait bound 与泛型参数声明在一起，位于尖括号中的冒号后面。

`impl Trait` 很方便，适用于短小的例子。trait bound 则适用于更复杂的场景。

例如，可以获取两个实现了 `Summary` 的参数，使用 `impl Trait` 的语法看起来像这样：

```rust
pub fn notify(item1: impl Summary, item2: impl Summary) {
```

这适用于 `item1` 和 `item2` 允许是不同类型的情况（只要他们都实现了 `Summary`）。不过，如我们希望它们都是相同类型呢？这只有在使用 trait bound 时才有可能：

```rust
pub fn notify<T: Summary>(item1: T, item2: T) {
```

泛型 `T` 被指定为 `item1` 和 `item2` 的参数限制，如此传递给参数 `item1` 和 `item2` 值的具体类型必须一致。

### 通过 + 指定多个 trait bound

如果 `notify` 需要显示 `item` 的格式化形式，同时也要使用 `summarize` 方法，那么 `item` 就需要同时实现两个不同的 trait：`Display` 和 `Summary`。这可以通过 `+` 语法实现。

```rust
pub fn my_notify(item: impl Summary + Display) {
pub fn my_notify<T: Summary + Display>(item: T) {
```
