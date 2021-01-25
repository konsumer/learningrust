/*
- Enums are kinda like structs, in that you can give them methods and keys can hold value
- Enums are an enumeration of possible values
- Option<T> is a special enum: Some(T) or None. T is the type of anything
*/

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// they can hold value
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    // can have sub-value of any type
    Quarter(UsState),
}

// match is kinda like switch/case for enum
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        // _ => (), // use this to match everything else (like default in switch)
    }
}

// add 1 to a Option<i32>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // need to handle both in match
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?} {:?}", four, six);
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?} {:?}", home, loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    // quick Option example, but will look closer at this, later
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y.expect("y has gotta be a i8"); // grab the actual value out of Option
    println!("{} {} {}", x, y.expect("even gotta expect here"), sum);

    println!("Penny: {}", value_in_cents(Coin::Penny));
    println!("Nickel: {}", value_in_cents(Coin::Nickel));
    println!("Dime: {}", value_in_cents(Coin::Dime));
    println!("Quarter: {}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five).expect("should be a num");
    // let none = plus_one(None).expect("commented out, becasue None");
    println!("six: {}", six);

    // shorter syntax for single match
    let y: Option<i8> = Some(3);
    if let Some(3) = y {
        println!("three");
    }
}
