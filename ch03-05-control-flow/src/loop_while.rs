// 可以使用 while 结构来遍历集合中的元素，比如数组。例如，看看示例 3-4:
fn loop_while(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
/*
数组中的所有五个元素都如期被打印出来。尽管 index 在某一时刻会到达值 5，不过循环在其尝试从数组获取第六个值（会越界）之前就停止了。

但这个过程很容易出错；如果索引长度或测试条件不正确会导致程序 panic。
例如，如果将 a 数组的定义改为包含 4 个元素而忘记了更新条件 while index < 4，则代码会 panic。这也使程序更慢，
因为编译器增加了运行时代码来对每次循环进行条件检查，以确定在循环的每次迭代中索引是否在数组的边界内。
 */