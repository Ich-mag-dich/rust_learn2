// #[derive(Debug)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// impl Message {
//     fn call(&self) {
//         println!("call: {:?}", self);
//     }
// }
// fn main() {
//     let home = IpAddr::V4(String::from("127.0.0.1"));

//     let loopback = IpAddr::V6(String::from("::1"));
//     println!("{:?} | {:?}", home, loopback);
//     let m = Message::Write(String::from("Hello"));
//     m.call();
// }

// match ---------------------------------------------------------------------
#[derive(Debug)]
enum UsState {
    Alabama,
    ALaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    println!("hey, {}", value_in_cents(coin));
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
} // TODO https://rust-kr.github.io/doc.rust-kr.org/ch06-02-match.html , 6.2장 하다 말았음 .
