fn mutable_reference() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用
    println!("second print! {} and {}", r1, r2);

    let r3 = &mut s; // 没问题
    println!("{}", r3);

}