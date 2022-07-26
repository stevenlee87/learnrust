#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}

/*
mkdir tests
cp lib2.rs tests/
cargo test --test lib2
   Compiling adder2 v0.1.0 (/Users/steven/project/learnrust/adder)
    Finished test [unoptimized + debuginfo] target(s) in 1.14s
     Running tests/lib2.rs (target/debug/deps/lib2-d1b972e890c903bb)

running 1 test
test tests2::larger_can_hold_smaller ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

 */