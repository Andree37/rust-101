enum IPAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => return 1,
        Coin::Nickel => return 5,
        Coin::Dime => return 10,
        // extract with match a value inside of an enum
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            return 25;
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => return None,
        Some(value) => return Some(value + 1),
    }
}

fn config_max() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn main() {
    let four = IPAddrKind::V4(127, 0, 0, 1);
    let six = IPAddrKind::V6(String::from("::1"));

    let absent: Option<i32> = None;
}
