struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

fn main() {
    let mut user1 = User {
        username: String::from("xfy"),
        email: String::from("i@rua.plus"),
        sign_in_count: 1,
        active: true,
    };

    println!("User1's email is {}", user1.email);
    user1.email = String::from("i@defect.ink");
    println!("User1's email is {}", user1.email);

    let user2 = build_user(String::from("i@rua.plus"), String::from("小肥羊"));
    println!("User2's email is {}", user2.email);

    let user3 = User {
        username: String::from("xfy"),
        email: String::from("i@defect.ink"),
        ..user2
    };
    println!("User2's email is {}", user2.username);

    let color1 = Color(255, 255, 255);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}
