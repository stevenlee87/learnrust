use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

/*
虽然这段代码有着如示例 9-5 一样的行为，但并没有包含任何 match 表达式且更容易阅读。在阅读完第十三章后再回到这个例子，
并查看标准库文档 unwrap_or_else 方法都做了什么操作。在处理错误时，还有很多这类方法可以消除大量嵌套的 match 表达式。
 */