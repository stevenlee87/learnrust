#![allow(unused)]
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
/*
在 main 函数中，定义了一个有 i32 类型的 x（其值为 5）和 f64 的 y（其值为 10.4）的 Point。
p2 则是一个有着字符串 slice 类型的 x（其值为 "Hello"）和 char 类型的 y（其值为c）的 Point。
在 p1 上以 p2 作为参数调用 mixup 会返回一个 p3，它会有一个 i32 类型的 x，因为 x 来自 p1，并拥有一个 char 类型的 y，
因为 y 来自 p2。println! 会打印出 p3.x = 5, p3.y = c。

这个例子的目的是展示一些泛型通过 impl 声明而另一些通过方法定义声明的情况。这里泛型参数 X1 和 Y1 声明于 impl 之后，
因为他们与结构体定义相对应。而泛型参数 X2 和 Y2 声明于 fn mixup 之后，因为他们只是相对于方法本身的。


 */