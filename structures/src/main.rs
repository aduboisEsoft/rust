fn main() {
    let user1 = build_user(String::from("elnono@gmail.com"), String::from("elnono"));

    let user2 = User {
        email:String::from("arnaud"),
        ..user1
    };

    let black = Color(1,0,0);
    
    println!("Salut {} et {}", black.0, user2.email);
}

fn build_user(email: String, username: String) -> User {
    User{
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64, 
}

struct Color(i32,i32,i32);