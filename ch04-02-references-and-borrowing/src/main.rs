include!("mutable_reference.rs");
include!("dange.rs");
/*
示例 4-5 中的元组代码有这样一个问题：我们必须将 String 返回给调用函数，以便在调用 calculate_length
后仍能使用 String，因为 String 被移动到了 calculate_length 内。相反我们可以提供一个 String 值的引用（reference）
引用（reference）像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据。
与指针不同，引用确保指向某个特定类型的有效值。下面是如何定义并使用一个（新的）calculate_length 函数，
它以一个对象的引用作为参数而不是获取值的所有权：
 */
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    mutable_reference();

    let reference_to_nothing = dangle();
    println!("reference_to_nothing is {}", reference_to_nothing)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn bigproblem(){
//     let mut s2 = String::from("hello");
//
//     let r1 = &s2; // 没问题
//     let r2 = &s2; // 没问题
//     let r3 = &mut s2; // 大问题
//
//     println!("{}, {}, and {}", r1, r2, r3);
// }