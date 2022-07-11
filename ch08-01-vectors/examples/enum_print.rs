#[derive(Debug)]
enum Message {
    Text(String),
    Number(i32),
}

fn main() {
    let msg = Message::Number(5);
    println!("{:?}", msg);
}