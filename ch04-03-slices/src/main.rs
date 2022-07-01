include!("str_code.rs");

fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("The value of hello is {}", hello);
    println!("The value of world is {}", world);

    str_code();
}

