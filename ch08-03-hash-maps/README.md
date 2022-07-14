## [哈希 map 储存键值对](https://kaisery.github.io/trpl-zh-cn/ch08-03-hash-maps.html#哈希-map-储存键值对)

> [ch08-03-hash-maps.md](https://github.com/rust-lang/book/blob/main/src/ch08-03-hash-maps.md)
> commit 1fd890031311612e54965f7f800a8c8bd4464663

最后介绍的常用集合类型是 **哈希 map**（*hash map*）。`HashMap<K, V>` 类型储存了一个键类型 `K` 对应一个值类型 `V` 的映射。它通过一个 **哈希函数**（*hashing function*）来实现映射，决定如何将键和值放入内存中。很多编程语言支持这种数据结构，不过通常有不同的名字：哈希、map、对象、哈希表或者关联数组，仅举几例。

哈希 map 可以用于需要任何类型作为键来寻找数据的情况，而不是像 vector 那样通过索引。例如，在一个游戏中，你可以将每个团队的分数记录到哈希 map 中，其中键是队伍的名字而值是每个队伍的分数。给出一个队名，就能得到他们的得分。

本章我们会介绍哈希 map 的基本 API，不过还有更多吸引人的功能隐藏于标准库在 `HashMap<K, V>` 上定义的函数中。一如既往请查看标准库文档来了解更多信息。

### [新建一个哈希 map](https://kaisery.github.io/trpl-zh-cn/ch08-03-hash-maps.html#新建一个哈希-map)

类似于 vector，哈希 map 是同质的：所有的键必须是相同类型，值也必须都是相同类型。