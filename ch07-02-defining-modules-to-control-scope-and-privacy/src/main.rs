/*
在前面我们提到了，src/main.rs 和 src/lib.rs 叫做 crate 根。之所以这样叫它们是因为这两个文件的内容都分别在
crate 模块结构的根组成了一个名为 crate 的模块，该结构被称为 模块树（module tree）。

示例 7-2 展示了示例 7-1 中的模块树的结构。
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
 */
fn main() {
    println!("Hello, world!");
}
