mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
/*
因为我们创建了名为 Appetizer 的公有枚举，所以我们可以在 eat_at_restaurant 中使用 Soup 和 Salad 成员。

如果枚举成员不是公有的，那么枚举会显得用处不大；给枚举的所有成员挨个添加 pub 是很令人恼火的，因此枚举成员默认就是公有的。
结构体通常使用时，不必将它们的字段公有化，因此结构体遵循常规，内容全部是私有的，除非使用 pub 关键字。
 */
