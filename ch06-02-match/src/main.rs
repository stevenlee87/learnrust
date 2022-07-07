use crate::Coin::Penny;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // match 的分支。一个分支有两个部分：一个模式和一些代码
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let value = value_in_cents(Penny);
    println!("value is {}", value);
}

/*
拆开 value_in_cents 函数中的 match 来看。首先，我们列出 match 关键字后跟一个表达式，在这个例子中是 coin 的值。
这看起来非常像 if 使用的表达式，不过这里有一个非常大的区别：对于 if，表达式必须返回一个布尔值，而这里它可以是任何类型的。
例子中的 coin 的类型是示例 6-3 中定义的 Coin 枚举。

接下来是 match 的分支。一个分支有两个部分：一个模式和一些代码。第一个分支的模式是值 Coin::Penny 而之后的 => 运算符
将模式和将要运行的代码分开。这里的代码就仅仅是值 1。每一个分支之间使用逗号分隔。

当 match 表达式执行时，它将结果值按顺序与每一个分支的模式相比较。如果模式匹配了这个值，这个模式相关联的代码将被执行。
如果模式并不匹配这个值，将继续执行下一个分支，非常类似一个硬币分类器。可以拥有任意多的分支：示例 6-3 中的 match 有四个分支。

每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值。
 */