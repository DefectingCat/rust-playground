另一个没有所有权的数据类型是 slice。slice 允许引用集合中一段连续的元素序列，而不用引用整个集合。

这里有一个编程小习题：编写一个函数，该函数接收一个字符串，并返回在该字符串中找到的第一个单词。如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。

首先创建一个函数 `first_word`，它的签名看上去应该是这样的：

```rust
fn first_word(s: &String) -> ?
```

因为我们不需要所有权，`first_word` 函数的第一个参数为 `&String`。我们并没有一个真正获取 部分 字符串的办法。不过，我们可以返回单词结尾的索引。

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

这里首先需要逐个检查 `String` 中的值释放为空格，需要用 `as_bytes` 方法将其转换为字节数组：

```rust
let bytes = s.as_bytes();
```

接下来，使用 iter 方法在字节数组上创建一个迭代器：

```rust
for (i, &item) in bytes.iter().enumerate() {
```

暂时先不考虑迭代器。现在，只需知道 iter 方法返回集合中的每一个元素，而 enumerate 包装了 iter 的结果，将这些元素作为元组的一部分来返回。enumerate 返回的元组中，第一个元素是索引，第二个元素是集合中元素的引用。这比我们自己计算索引要方便一些。

因为 enumerate 方法返回一个元组，我们可以使用模式来解构。所以在 `for` 循环中，我们指定了一个模式，其中元组中的 `i` 是索引而元组中的 `&item` 是单个字节。因为我们从 `.iter().enumerate()` 中获取了集合元素的引用，所以模式中使用了 &。

在 for 循环中，我们通过字节的字面值语法来寻找代表空格的字节。如果找到了一个空格，返回它的位置。否则，使用 s.len() 返回字符串的长度：

```rust
for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
        return i;
    }
}

s.len()
```

但现在我们只是能获取到空格的下标位置，并且返回的位置与字符串本身并不同步，因为它只是我们找到的下标。如果字符串被清空或修改，返回的下标依然不变。

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word 的值为 5

    s.clear(); // 这清空了字符串，使其等于 ""

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
}
```

这个程序编译时没有任何错误，而且在调用 s.clear() 之后使用 word 也不会出错。因为 word 与 s 状态完全没有联系，所以 word 仍然包含值 5。可以尝试用值 5 来提取变量 s 的第一个单词，不过这是有 bug 的，因为在我们将 5 保存到 word 之后 s 的内容已经改变。

## 字符串 slice

**字符串 slice**（string slice）是 String 中一部分值的引用，它看起来像这样：

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

这类似于引用整个 `String`，不过带有额外的 `[0..5]` 的部分。它不是对整个 `String` 的引用，而是针对部分 `String` 的引用。

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
```

在了解到 slice 后，就可以创建一个字符串 slice 并返回。但这样做相比之前返回索引的做法好处是什么呢？

现在这样做，编译器会确保指向 String 的引用持续有效。

之前的做法，当我们获取第一个单词结尾的索引后，接着就清除了字符串就会出现导致索引就无效的 bug。那些代码在逻辑上是不正确的，但却没有显示任何直接的错误。问题会在之后尝试对空字符串使用第一个单词的索引时出现。slice 就不可能出现这种 bug 并让我们更早的知道出问题了。使用 slice 版本的 first_word 会抛出一个编译时错误：

```rust
fn main() {
    let mut s = String::from("hello world");

    let first = first_word(&s);

    s.clear(); // cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{}", first)
}
```

回忆一下借用规则，当拥有某值的不可变引用时，就不能再获取一个可变引用。因为 `clear` 需要清空 `String`，它尝试获取一个可变引用。在调用 `clear` 之后的 `println!` 使用了 `first` 中的引用，所以这个不可变的引用在此时必须仍然有效。Rust 不允许 clear 中的可变引用和 `first` 中的不可变引用同时存在，因此编译失败。Rust 不仅使得我们的 API 简单易用，也在编译时就消除了一整类的错误！

### 字符串字面值就是 slice

```rust
let s = "Hello, world!";
```

这里 s 的类型是 &str：它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的；&str 是一个不可变引用。

### 字符串 slice 作为参数

如果有一个字符串 `slice`，可以直接传递它。如果有一个 `String`，则可以传递整个 `String` 的 `slice` 或对 `String` 的引用。这种灵活性利用了 deref coercions 的优势。定义一个获取字符串 `slice` 而不是 `String` 引用的函数使得我们的 API 更加通用并且不会丢失任何功能：

```rust
fn main() {
    let mut s = String::from("hello world");

    let first = first_word(&s[..]);
    println!("{}", first);
    s.clear();
    // println!("{}", first)
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
```

```rust
fn main() {
    let my_string = String::from("hello world");

    // first_word 中传入 `String` 的 slice
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word 中传入字符串字面值的 slice
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值 **就是** 字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word(my_string_literal);
}
```

## 其他类型的 slice

字符串 slice 是针对字符串的，不过也有更通用的 slice 类型。就跟我们想要获取字符串的一部分那样，我们也会想要引用数组的一部分。我们可以这样做：

```rust

let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

这个 slice 的类型是 `&[i32]`。它跟字符串 slice 的工作方式一样，通过存储第一个集合元素的引用和一个集合总长度。

## 小结

所有权、借用和 slice 这些概念让 Rust 程序在编译时确保内存安全。Rust 语言提供了跟其他系统编程语言相同的方式来控制你使用的内存，但拥有数据所有者在离开作用域后自动清除其数据的功能意味着你无须额外编写和调试相关的控制代码。