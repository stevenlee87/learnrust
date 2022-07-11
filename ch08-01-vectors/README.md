## [vector 用来储存一系列的值](https://kaisery.github.io/trpl-zh-cn/ch08-01-vectors.html#vector-用来储存一系列的值)

> [ch08-01-vectors.md](https://github.com/rust-lang/book/blob/main/src/ch08-01-vectors.md)
> commit e7bfb353b107cb150faab9d331c99ea2b91f3725

我们要讲到的第一个类型是 `Vec<T>`，也被称为 *vector*。vector 允许我们在一个单独的数据结构中储存多于一个的值，它在内存中彼此相邻地排列所有的值。vector 只能储存相同类型的值。它们在拥有一系列项的场景下非常实用，例如文件中的文本行或是购物车中商品的价格。

### [新建 vector](https://kaisery.github.io/trpl-zh-cn/ch08-01-vectors.html#新建-vector)

为了创建一个新的空 vector，可以调用 `Vec::new` 函数，如示例 8-1 所示：

```rust
    let v: Vec<i32> = Vec::new();
```

示例 8-1：新建一个空的 vector 来储存 `i32` 类型的值

注意这里我们增加了一个类型注解。因为没有向这个 vector 中插入任何值，Rust 并不知道我们想要储存什么类型的元素。这是一个非常重要的点。vector 是用泛型实现的，第十章会涉及到如何对你自己的类型使用它们。现在，<u>所有你需要知道的就是 `Vec<T>` 是一个由标准库提供的类型，它可以存放任何类型，而当 `Vec` 存放某个特定类型时，那个类型位于尖括号中。</u>在示例 8-1 中，我们告诉 Rust `v` 这个 `Vec<T>` 将存放 `i32` 类型的元素。

通常，我们会用初始值来创建一个 `Vec<T>` 而 Rust 会推断出储存值的类型，所以很少会需要这些类型注解。为了方便 Rust 提供了 `vec!` 宏，这个宏会根据我们提供的值来创建一个新的 vector。示例 8-2 新建一个拥有值 `1`、`2` 和 `3` 的 `Vec<i32>`。推断为 `i32` 是因为这是默认整型类型，第三章的 [“数据类型”](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#数据类型) 讨论过：

```rust
    let v = vec![1, 2, 3];
```

示例 8-2：新建一个包含初值的 vector