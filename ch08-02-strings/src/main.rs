fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is: {}", s)

    /*
    这些代码也会将 s 设置为 “tic-tac-toe”。format! 与 println! 的工作原理相同，不过不同于将输出打印到屏幕上，
    它返回一个带有结果内容的 String。这个版本就好理解的多，宏 format! 生成的代码使用索引并且不会获取任何参数的所有权。
     */
}
