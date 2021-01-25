/*
- the entire instance must be mutable, if you want that, no single field
*/

// lifetime parameters are needed for structs with refs, those will be discussed later, this errors:
// struct UserBad {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

#[allow(dead_code)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// struct shorthand works like ES6
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// it also uses .. like JS's ...
fn build_user2(email: String, username: String) -> User {
    let userbase = User{
        email: String::from("NONE"),
        username: String::from("NONE"),
        active: true,
        sign_in_count: 1,
    };
    User {
        email,
        username,
        ..userbase
    }
}

// use this to make a struct printable with {:?}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// add methods like this
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


fn main() {
    // not mutable
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1.email);

    // mutable
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);

    let user1 = build_user(String::from("someone@example.com"), String::from("someone"));
    println!("{}", user1.email);

    let user1 = build_user2(String::from("someone@example.com"), String::from("someone"));
    println!("{}", user1.email);

    // tupe-structs are a way to make a named tuple-type to differntiate it from random tuples
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}, {}, {}", black.0, black.1, black.2);
    println!("{}, {}, {}", origin.0, origin.1, origin.2);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}