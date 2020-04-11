#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println!("***Structs*** ");
    let mut user1 = User {
        username : String::from("Yelnats"),
        email : String::from("stanley@gmail.com"),
        sign_in_count: 25,
        active: true,
    };
    
    user1.email = String::from("stanley@yelnats.com");
    println!("username = {}", user1.username);

    let user2 = build_user(String::from("user2@gmail.com"), String::from("user2"));
    println!("User 2 email = {}", user2.email);

    // Use struct update syntax to use same values from user2
    let user3 = User {
        email: String::from("user3@outlook.com"),
        username: String::from("Homer Simpson"),
        ..user2
    };

    println!("Debug of user3 = {:?}", user3);


}

fn build_user(email: String, username: String) -> User {
    // Use the field init shorthand for fields that match param names exactly
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
