**内部可变性**（_Interior mutability_）是 Rust 中的一个设计模式，它允许你即使在有不可变引用时也可以改变数据，这通常是借用规则所不允许的。当可以确保代码在运行时会遵守借用规则，即使编译器不能保证的情况，可以选择使用那些运用内部可变性模式的类型。所涉及的 unsafe 代码将被封装进安全的 API 中，而外部类型仍然是不可变的。

## 通过 `RefCell<T>` 在运行时检查借用规则

不同于 `Rc<T>`，`RefCell<T>` 代表其数据的唯一的所有权。那么是什么让 `RefCell<T>` 不同于像 `Box<T>` 这样的类型呢？

借用规则：

1. 在任意给定时刻，只能拥有一个可变引用或任意数量的不可变引用 之一（而不是两者）。
2. 引用必须总是有效的。

对于引用和 `Box<T>，`借用规则的不可变性作用于编译时。对于 `RefCell<T>`，这些不可变性作用于 **运行时**。对于引用，如果违反这些规则，会得到一个编译错误。而对于 `RefCell<T>`，如果违反这些规则程序会 panic 并退出。

在编译时检查借用规则的优势是这些错误将在开发过程的早期被捕获，同时对运行时没有性能影响，因为所有的分析都提前完成了。为此，在编译时检查借用规则是大部分情况的最佳选择，这也正是其为何是 Rust 的默认行为。

相反在运行时检查借用规则的好处则是允许出现特定内存安全的场景，而它们在编译时检查中是不允许的。静态分析，正如 Rust 编译器，是天生保守的。但代码的一些属性不可能通过分析代码发现：其中最著名的就是 [停机问题（Halting Problem）](https://zh.wikipedia.org/wiki/%E5%81%9C%E6%9C%BA%E9%97%AE%E9%A2%98)啊。

因为一些分析是不可能的，如果 Rust 编译器不能通过所有权规则编译，它可能会拒绝一个正确的程序；从这种角度考虑它是保守的。如果 Rust 接受不正确的程序，那么用户也就不会相信 Rust 所做的保证了。然而，如果 Rust 拒绝正确的程序，虽然会给程序员带来不便，但不会带来灾难。`RefCell<T>` 正是用于当你确信代码遵守借用规则，而编译器不能理解和确定的时候。

类似于 `Rc<T>`，`RefCell<T>` 只能用于单线程场景。如果尝试在多线程上下文中使用`RefCell<T>`，会得到一个编译错误。

如下为选择 `Box<T>`，`Rc<T>` 或 `RefCell<T>` 的理由：

- `Rc<T>` 允许相同数据有多个所有者；`Box<T>` 和 `RefCell<T>` 有单一所有者。
- `Box<T>` 允许在编译时执行不可变或可变借用检查；`Rc<T>`仅允许在编译时执行不可变借用检查；`RefCell<T>` 允许在运行时执行不可变或可变借用检查。
- 因为 `RefCell<T>` 允许在运行时执行可变借用检查，所以我们可以在即便 `RefCell<T>` 自身是不可变的情况下修改其内部的值。

在不可变值内部改变值就是 **内部可变性** 模式。让我们看看何时内部可变性是有用的，并讨论这是如何成为可能的。

### 内部可变性：不可变值的可变借用

借用规则的一个推论是当有一个不可变值时，不能可变地借用它。例如，如下代码不能编译：

```rust
fn main() {
    let x = 5;
    let y = &mut x;
}
```

如果尝试编译，会得到如下错误：

```bash
``error[E0596]: cannot borrow immutable local variable `x` as mutable
 --> src/main.rs:3:18
 |
2 | let x = 5;
 | - consider changing this to `mut x`
3 | let y = &mut x;
 | ^ cannot borrow mutably``
```

然而，特定情况下，令一个值在其方法内部能够修改自身，而在其他代码中仍视为不可变，是很有用的。值方法外部的代码就不能修改其值了。`RefCell<T>` 是一个获得内部可变性的方法。`RefCell<T>` 并没有完全绕开借用规则，编译器中的借用检查器允许内部可变性并相应地在运行时检查借用规则。如果违反了这些规则，会出现 panic 而不是编译错误。

#### 内部可变性用例：mock 对象

**测试替身**（*test double*）是一个通用编程概念，它代表一个在测试中替代某个类型的类型。**mock 对象** 是特定类型的测试替身，它们记录测试过程中发生了什么以便可以断言操作是正确的。

虽然 Rust 中的对象与其他语言中的对象并不是一回事，Rust 也没有像其他语言那样在标准库中内建 mock 对象功能，不过我们确实可以创建一个与 mock 对象有着相同功能的结构体。

如下是一个我们想要测试的场景：我们在编写一个记录某个值与最大值的差距的库，并根据当前值与最大值的差距来发送消息。例如，这个库可以用于记录用户所允许的 API 调用数量限额。

首先，假设我们有个实现了该发送消息 trait 的类型，当我们的 `LimitChecker` 检查了当前值与最大值之后，就调用该方法来发送消息。具体发送的方式此处不关注，主要的是该 trait 没有获取 `self` 的可变引用。

```rust
pub trait Messager {
    fn sent(&self, msg: str);
}
```

我们的 `LimitChecker` 实现如下：

```rust
pub struct LimitChecker<'a, T> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitChecker<'a, T>
where
    T: Messager,
{
    pub fn new(messager: &'a T, max: usize) -> Self<T> {
        Self {
            messager,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_value = self.value as f64 / self.max as f64;

        if percentage_of_value >= 1.0 {
            self.messager.sent("Error: You are over your quota!")
        } else if percentage_of_value >= 0.9 {
            self.messager
                .sent("Urgent warning: You've used up over 90% of your quota!")
        } else if percentage_of_value >= 0.75 {
            self.messager
                .sent("Warning: You've used up over 75% of your quota!")
        }
    }
}
```

我们所需的 mock 对象是，调用 `send` 并不实际发送 email 或消息，而是只记录信息被通知要发送了。可以新建一个 mock 对象实例，用其创建 `LimitTracker`，调用 `LimitTracker` 的 `set_value` 方法，然后检查 mock 对象是否有我们期望的消息。

```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessager {
        messages: Vec<String>,
    }

    impl MockMessager {
        fn new() -> Self {
            Self { messages: vec![] }
        }
    }

    impl Messager for MockMessager {
        fn sent(&self, msg: &str) {
            self.messages.push(msg.to_string())    
        }
    }
    
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock = MockMessager::new();
        let mut checker = LimitChecker::new(&mock, 100);
        checker.set_value(80);

        assert_eq!(mock.messages.len(), 1)
    }
}


```

为了测试，我们并不实际发送消息，消息只是存在 `MockMessager` 内部，用于检测 `set_value()` 是否如期望的工作。问题的关键就在这里，发送消息的 trait 获取的是不可变的引用。而我们如果需要修改 `MockMessager` 就需要一个可变引用！

```bash
error[E0596]: cannot borrow `self.messages` as mutable, as it is behind a `&` reference
  --> src\main.rs:58:13
   |
2  |     fn sent(&self, msg: &str);
   |             ----- help: consider changing that to be a mutable reference: `&mut self`
...
58 |             self.messages.push(msg.to_string())
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
```

不能修改 `MockMessenger` 来记录消息，因为 `send` 方法获取了 `self` 的不可变引用。我们也不能参考错误文本的建议使用 `&mut self` 替代，因为这样 `send` 的签名就不符合 `Messenger` trait 定义中的签名了。

这正是内部可变性的用武之地！我们将通过 `RefCell` 来储存 `sent_messages`，然后 `send` 将能够修改 `sent_messages` 并储存消息。

```rust
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock = MockMessager::new();
        let mut checker = LimitChecker::new(&mock, 100);
        checker.set_value(80);

        assert_eq!(mock.messages.borrow().len(), 1);
    }
```

现在 `sent_messages` 字段的类型是 `RefCell<Vec<String>>` 而不是 `Vec<String>`。在 `new` 函数中新建了一个 `RefCell<Vec<String>>` 实例替代空 vector。

对于 `send` 方法的实现，第一个参数仍为 `self` 的不可变借用，这是符合方法定义的。我们调用 `self.sent_messages` 中 `RefCell` 的 `borrow_mut` 方法来获取 `RefCell` 中值的可变引用，这是一个 vector。接着可以对 vector 的可变引用调用 `push` 以便记录测试过程中看到的消息。

最后必须做出的修改位于断言中：为了看到其内部 vector 中有多少个项，需要调用 `RefCell` 的 `borrow` 以获取 vector 的不可变引用。
