// 如果存在嵌套循环，break 和 continue 应用于此时最内层的循环。你可以选择在一个循环上指定一个 循环标签（loop label），
// 然后将标签与 break 或 continue 一起使用，使这些关键字应用于已标记的循环而不是最内层的循环。
// 下面是一个包含两个嵌套循环的示例:
fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
/*
外层循环有一个标签 counting_up，它将从 0 数到 2。没有标签的内部循环从 10 向下数到 9。
第一个没有指定标签的 break 将只退出内层循环。break 'counting_up; 语句将退出外层循环。
 */