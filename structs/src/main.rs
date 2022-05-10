struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("test@example.com"),
        username: String::from("test"),
        sign_in_count: 1,
        active: true,
    };
    user1.email = String::from("another@example.com");

    // we can use a struct update syntax to create user2 with some values from user1
    let user2 = User {
        email: String::from("test2@example.com"),
        ..user1 // we can no longer use user1 'moveable stuff' after this
    };

    // this returns an error
    //println!("{}", user1.username);
    // this doesnt, as ints are copied, never moved
    println!("{}", user1.active);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
