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

    println!("The value of user1.email: {}", user1.email);
    println!("The value of user1.active: {}", user1.active);
    println!("The value of user1.username: {}", user1.username);
    println!("The value of user1.sign_in_count: {}", user1.sign_in_count);

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("user2@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("The value of user2.email: {}", user2.email);
    println!("The value of user2.active: {}", user2.active);
    println!("The value of user2.username: {}", user2.username);
    println!("The value of user2.sign_in_count: {}", user2.sign_in_count);

    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2
    };
    println!("The value of user3.email: {}", user3.email);
    println!("The value of user3.active: {}", user3.active);
    println!("The value of user3.username: {}", user3.username);
    println!("The value of user3.sign_in_count: {}", user3.sign_in_count);
}
