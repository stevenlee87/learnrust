### [通过派生 trait 增加实用功能](https://kaisery.github.io/trpl-zh-cn/ch05-02-example-structs.html#通过派生-trait-增加实用功能)

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);
}
```

