use std::thread;

// 示例 16-5: 使用 move 关键字强制获取它使用的值的所有权
fn main() {
    let v = vec![1, 2, 3];

    // Rust 会 推断 如何捕获 v，因为 println! 只需要 v 的引用，闭包尝试借用 v。然而这有一个问题：Rust 不知道这个
    // 新建线程会执行多久，所以无法知晓 v 的引用是否一直有效。
    // 所以需要使用move 关键字
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

/*
move 关键字经常用于传递给 thread::spawn 的闭包，因为闭包会获取从环境中取得的值的所有权，因此会将这些值的所有权从一
个线程传送到另一个线程。在第十三章 “闭包会捕获其环境” 部分讨论了闭包上下文中的 move。
现在我们会更专注于 move 和 thread::spawn 之间的交互。
 */