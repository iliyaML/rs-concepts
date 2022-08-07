#![allow(unused)]

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    // shorthand
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// tuple structs
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

// unit struct
struct AlwaysEqual;

fn main() {
    // initialize struct by passing all the values,
    // order does not matter
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // to edit user1, make it mutable
    let mut user1 = user1;
    user1.email = String::from("anotheremail@example.com");

    // pretty print struct
    println!("user1: {user1:#?}");

    // use function to build user
    let user2 = build_user(
        String::from("newemail@example.com"),
        String::from("newuser"),
    );

    println!("user2: {user2:#?}");

    let user3 = User {
        email: String::from("brandnewuser@example.com"),
        username: String::from("brandnewuser"),
        ..user1
    };

    println!("user3: {user3:#?}");

    // tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{black:?}");
    println!("{origin:?}");

    // unit struct
    let subject = AlwaysEqual;
}
