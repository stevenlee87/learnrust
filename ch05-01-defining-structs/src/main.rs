struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    println!("The value of user1.email: {}",user1.email);
    println!("The value of user1.active: {}",user1.active);
    println!("The value of user1.username: {}",user1.username);
    println!("The value of user1.sign_in_count: {}",user1.sign_in_count);
}