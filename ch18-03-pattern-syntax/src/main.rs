fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

/*
Matched, y = 5
at the end: x = Some(5), y = 10

让我们看看当 match 语句运行的时候发生了什么。第一个匹配分支的模式并不匹配 x 中定义的值，所以代码继续执行。

第二个匹配分支中的模式引入了一个新变量 y，它会匹配任何 Some 中的值。因为我们在 match 表达式的新作用域中，
这是一个新变量，而不是开头声明为值 10 的那个 y。这个新的 y 绑定会匹配任何 Some 中的值，在这里是 x 中的值。
因此这个 y 绑定了 x 中 Some 内部的值。这个值是 5，所以这个分支的表达式将会执行并打印出 Matched, y = 5。

如果 x 的值是 None 而不是 Some(5)，头两个分支的模式不会匹配，所以会匹配下划线。这个分支的模式中没有引入变量 x，
所以此时表达式中的 x 会是外部没有被覆盖的 x。在这个假想的例子中，match 将会打印 Default case, x = None。

一旦 match 表达式执行完毕，其作用域也就结束了，同理内部 y 的作用域也结束了。
最后的 println! 会打印 at the end: x = Some(5), y = 10。

为了创建能够比较外部 x 和 y 的值，而不引入覆盖变量的 match 表达式，我们需要相应地使用带有条件的
匹配守卫（match guard）。我们稍后将在 “匹配守卫提供的额外条件” 这一小节讨论匹配守卫。
 */