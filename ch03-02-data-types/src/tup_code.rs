fn tup_code() {
    let tup = (500, 6.4, 1);

    let (t, u, p) = tup;

    println!("The value of t is: {}", t);
    println!("The value of u is: {}", u);
    println!("The value of p is: {}", p);

    /*
    程序首先创建了一个元组并绑定到 tup 变量上。接着使用了 let 和一个模式将 tup 分成了三个不同的变量，x、y 和 z。
    这叫做 解构（destructuring），因为它将一个元组拆成了三个部分。最后，程序打印出了 y 的值，也就是 6.4。

    我们也可以使用点号（.）后跟值的索引来直接访问它们。例如：
     */
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of five_hundred,six_point_four,one is: {},{},{}", five_hundred,six_point_four,one );

}