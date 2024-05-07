use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}

/*
模块小抄
这里我们提供一个简单的参考，用来解释模块、路径、use关键词和pub关键词如何在编译器中工作，以及大部分开发者如何组织他们的代码。我们将在本章节中举例说明每条规则，不过这是一个解释模块工作方式的良好参考。

从 crate 根节点开始: 当编译一个 crate, 编译器首先在 crate 根文件（通常，对于一个库 crate 而言是src/lib.rs，对于一个二进制 crate 而言是src/main.rs）中寻找需要被编译的代码。
声明模块: 在 crate 根文件中，你可以声明一个新模块；比如，你用mod garden;声明了一个叫做garden的模块。编译器会在下列路径中寻找模块代码：
内联，在大括号中，当mod garden后方不是一个分号而是一个大括号
在文件 src/garden.rs
在文件 src/garden/mod.rs
声明子模块: 在除了 crate 根节点以外的其他文件中，你可以定义子模块。比如，你可能在src/garden.rs中定义了mod vegetables;。编译器会在以父模块命名的目录中寻找子模块代码：
内联，在大括号中，当mod vegetables后方不是一个分号而是一个大括号
在文件 src/garden/vegetables.rs
在文件 src/garden/vegetables/mod.rs
模块中的代码路径: 一旦一个模块是你 crate 的一部分，你可以在隐私规则允许的前提下，从同一个 crate 内的任意地方，通过代码路径引用该模块的代码。举例而言，一个 garden vegetables 模块下的Asparagus类型可以在crate::garden::vegetables::Asparagus被找到。
私有 vs 公用: 一个模块里的代码默认对其父模块私有。为了使一个模块公用，应当在声明时使用pub mod替代mod。为了使一个公用模块内部的成员公用，应当在声明前使用pub。
use 关键字: 在一个作用域内，use关键字创建了一个成员的快捷方式，用来减少长路径的重复。在任何可以引用crate::garden::vegetables::Asparagus的作用域，你可以通过 use crate::garden::vegetables::Asparagus;创建一个快捷方式，然后你就可以在作用域中只写Asparagus来使用该类型。
这里我们创建一个名为backyard的二进制 crate 来说明这些规则。该 crate 的路径同样命名为backyard，该路径包含了这些文件和目录：

backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
这个例子中的 crate 根文件是src/main.rs，该文件包括了：

文件名：src/main.rs

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
pub mod garden;行告诉编译器应该包含在src/garden.rs文件中发现的代码：

文件名：src/garden.rs

pub mod vegetables;
在此处， pub mod vegetables;意味着在src/garden/vegetables.rs中的代码也应该被包括。这些代码是：

#[derive(Debug)]
pub struct Asparagus {}
 */
