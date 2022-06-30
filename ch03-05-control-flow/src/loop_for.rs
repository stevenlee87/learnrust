fn loop_for(){
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}

/*
当运行这段代码时，将看到与示例 3-4 一样的输出。更为重要的是，我们增强了代码安全性，并消除了可能由于超出数组的结尾或遍历长度不够而缺少一些元素而导致的 bug。

例如，在示例 3-4 的代码中，如果你将 a 数组的定义改为有四个元素，但忘记将条件更新为 while index < 4，
代码将会 panic。使用 for 循环的话，就不需要惦记着在改变数组元素个数时修改其他的代码了。

for 循环的安全性和简洁性使得它成为 Rust 中使用最多的循环结构。即使是在想要循环执行代码特定次数时，
例如示例 3-3 中使用 while 循环的倒计时例子，大部分 Rustacean 也会使用 for 循环。这么做的方式是使用 Range，
它是标准库提供的类型，用来生成从一个数字开始到另一个数字之前结束的所有数字的序列。
 */

fn loop_for_rev() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}