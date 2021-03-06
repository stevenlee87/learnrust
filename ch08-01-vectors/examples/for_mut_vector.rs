fn main() {
    let mut v = vec![100, 32, 57];
    // 为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值。
    // 第十五章的 “通过解引用运算符追踪指针的值” 部分会详细介绍解引用运算符。
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }
}