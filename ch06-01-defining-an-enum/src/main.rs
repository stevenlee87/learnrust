fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // 我们可以使用一种更简洁的方式来表达相同的概念，仅仅使用枚举并将数据直接放进每一个枚举成员而不是将枚举作为结构体
    // 的一部分。IpAddr 枚举的新定义表明了 V4 和 V6 成员都关联了 String 值：
    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }
    //
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    //
    // let loopback = IpAddr::V6(String::from("::1"));
}