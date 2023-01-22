#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr1 {
    kind: IpAddrKind,
    addr: String,
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// easy way
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

// hard way
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
} // regular struct
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        //_ => None,
        Some(i) => Some(i + 1),
    }
}

pub fn main5() {
    // structs chapter 5
    println!("\nchapter 6, enums");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(IpAddrKind::V6);
    let home1 = IpAddr1 {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };
    let loopback1 = IpAddr1 {
        kind: IpAddrKind::V6,
        addr: String::from("::1"),
    };

    let home2 = IpAddr2::V4(127, 0, 0, 1);
    let loopback2 = IpAddr2::V6(String::from("::1"));

    let m = Message::Write(String::from("hey ho"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    //let sum = x + y;

    let acoin = Coin::Quarter(UsState::Alaska);
    println!("coin value = {}", value_in_cents(&acoin));

    let five = Some(5);
    let size = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        _ => (),
    }

    let mut count = 0;
    if let Coin::Quarter(state) = acoin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

fn route(ip_kind: IpAddrKind) {}
