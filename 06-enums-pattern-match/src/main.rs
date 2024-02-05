#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn test_enum() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:#?}\n{:#?}", home, loopback);
}

#[derive(Debug)]
enum Message {
    Quit,
    // Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}

fn test_enum2() {
    let q = Message::Quit;
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(0xff, 0, 0);
    q.call();
    w.call();
    c.call();
}

fn test_option() {
    let some_number = Some(3);
    let some_string = Some("A String");
    let absent_number: Option<i32> = None;
    println!("{:#?}", absent_number);
    println!("{}", some_string.unwrap());
    let x = 42;
    let sum = x + some_number.unwrap();
    println!("{sum}");
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    //Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn test_match() {
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("{}", value_in_cents(Coin::Penny));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn test_match_option() {
    let five = Some(5);
    println!("{:?}", plus_one(five));
    println!("{:?}", plus_one(None));
}

fn test_match_placeholder() {
    let v = 0u8;
    match v {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        _ => (),
    }
}

fn test_if_let() {
    let v = Some(7u8);

    if let Some(0xff) = v {
        println!("seven");
    } else {
        print!("miss");
    }
}

fn main() {
    test_enum();
    test_enum2();
    test_option();
    test_match();
    test_match_option();
    test_match_placeholder();
    test_if_let();
}
