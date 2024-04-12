#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    user1.email = String::from("anotheremail@example.com");
    
    println!("{:?}", user1);
    println!("{:#?}", user1);
    
    let user2 = build_user("toto@gmail.com".to_string(), "toto".to_string());
    
    println!("{:?}", user2);
    
    let user3 = User {
        email: String::from("yetanother@example.com"),
        ..user2
    };
    
    println!("{:?}", user3);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("{:?}", origin);
}
