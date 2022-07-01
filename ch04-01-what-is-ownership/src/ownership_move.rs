fn ownership_move() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}

/*
如果你在其他语言中听说过术语 浅拷贝（shallow copy）和 深拷贝（deep copy），那么拷贝指针、长度和容量而不拷贝数据可能
听起来像浅拷贝。不过因为 Rust 同时使第一个变量无效了，这个操作被称为 移动（move），而不是浅拷贝。
上面的例子可以解读为 s1 被 移动 到了 s2 中。那么具体发生了什么，如图 4-4 所示。
 */