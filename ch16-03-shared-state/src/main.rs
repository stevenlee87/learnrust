use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
/*
示例 16-12: 出于简单的考虑，在一个单线程上下文中探索 Mutex<T> 的 API

像很多类型一样，我们使用关联函数 new 来创建一个 Mutex<T>。使用 lock 方法获取锁，以访问互斥器中的数据。
这个调用会阻塞当前线程，直到我们拥有锁为止。

如果另一个线程拥有锁，并且那个线程 panic 了，则 lock 调用会失败。在这种情况下，没人能够再获取锁，
所以这里选择 unwrap 并在遇到这种情况时使线程 panic。

一旦获取了锁，就可以将返回值（在这里是num）视为一个其内部数据的可变引用了。类型系统确保了我们在使用 m 中的值之前获取锁
：Mutex<i32> 并不是一个 i32，所以 必须 获取锁才能使用这个 i32 值。我们是不会忘记这么做的，
因为反之类型系统不允许访问内部的 i32 值。

正如你所怀疑的，Mutex<T> 是一个智能指针。更准确的说，lock 调用 返回 一个叫做 MutexGuard 的智能指针。这个智能指针实
现了 Deref 来指向其内部数据；其也提供了一个 Drop 实现当 MutexGuard 离开作用域时自动释放锁，这正发生于示例 16-12
内部作用域的结尾。为此，我们不会忘记释放锁并阻塞互斥器为其它线程所用的风险，因为锁的释放是自动发生的。

丢弃了锁之后，可以打印出互斥器的值，并发现能够将其内部的 i32 改为 6。


 */