fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
/*
这会打印出 {"world": 2, "hello": 1, "wonderful": 1}。split_whitespace 方法会迭代 text 的值由空格分隔的子 slice。
or_insert 方法返回这个键的值的一个可变引用（&mut V）。这里我们将这个可变引用储存在 count 变量中，
所以为了赋值必须首先使用星号（*）解引用 count。这个可变引用在 for 循环的结尾离开作用域，
这样所有这些改变都是安全的并符合借用规则。
 */