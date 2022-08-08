fn main() {
    let name1 = String::from("Fred");
    println!("name1: {}", name1);

    let name2 = name1;
    println!("name2: {}", name2);

    // 编译出错，这句会出错
    println!("name1 again: {}", name1);
}