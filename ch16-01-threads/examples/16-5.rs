use std::thread;

// 示例 16-5: 使用 move 关键字强制获取它使用的值的所有权
fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}