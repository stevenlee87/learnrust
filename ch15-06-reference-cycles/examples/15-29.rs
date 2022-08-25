// 示例 15-29：在内部作用域创建 branch 并检查其强弱引用计数
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

/*
leaf strong = 1, weak = 0
branch strong = 1, weak = 1
leaf strong = 2, weak = 0
leaf parent = None
leaf strong = 1, weak = 0

一旦创建了 leaf，其 Rc<Node> 的强引用计数为 1，弱引用计数为 0。在内部作用域中创建了 branch 并与 leaf 相关联，
此时 branch 中 Rc<Node> 的强引用计数为 1，弱引用计数为 1（因为 leaf.parent 通过 Weak<Node> 指向 branch）。
这里 leaf 的强引用计数为 2，因为现在 branch 的 branch.children 中储存了 leaf 的 Rc<Node> 的拷贝，不过弱引用计数仍然为 0。

当内部作用域结束时，branch 离开作用域，Rc<Node> 的强引用计数减少为 0，所以其 Node 被丢弃。
来自 leaf.parent 的弱引用计数 1 与 Node 是否被丢弃无关，所以并没有产生任何内存泄漏！

如果在内部作用域结束后尝试访问 leaf 的父节点，会再次得到 None。在程序的结尾，leaf 中 Rc<Node> 的强引用计数为 1，
弱引用计数为 0，因为现在 leaf 又是 Rc<Node> 唯一的引用了。

所有这些管理计数和值的逻辑都内建于 Rc<T> 和 Weak<T> 以及它们的 Drop trait 实现中。通
过在 Node 定义中指定从子节点到父节点的关系为一个Weak<T>引用，就能够拥有父节点和子节点之间的双向引用而不会造成引用循环和内存泄漏。
 */