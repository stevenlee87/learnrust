fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {}", first);
}

/*
示例 8-7 中的代码看起来应该能够运行：为什么第一个元素的引用会关心 vector 结尾的变化？
不能这么做的原因是由于 vector 的工作方式：在 vector 的结尾增加新元素时，
在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。
这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。

注意：关于 Vec<T> 类型的更多实现细节，请查看 “The Rustonomicon”
 */