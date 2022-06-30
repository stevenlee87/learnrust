/*
Rust 代码中的函数和变量名使用 snake case 规范风格。在 snake case 中，所有字母都是小写并使用下划线分隔单词。
这是一个包含函数定义示例的程序：
*/
fn main() {
    println!("Hello, world!");

    another_function();

    /*
    函数调用是一个表达式。宏调用是一个表达式。用大括号创建的一个新的块作用域也是一个表达式，例如：
     */
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let z = plus_one(5);

    println!("The value of z is: {}", z);
}

fn another_function() {
    println!("Another function.");
}
/*
这个表达式：
{
    let x = 3;
    x + 1
}
是一个代码块，它的值是 4。这个值作为 let 语句的一部分被绑定到 y 上。注意 x+1 这一行在结尾没有分号，
与你见过的大部分代码行不同。表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。
在接下来探索具有返回值的函数和表达式时要谨记这一点。
 */

fn plus_one(z: i32) -> i32 {
    z + 1
}