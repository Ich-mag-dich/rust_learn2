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
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     ALaska,
//     // --snip--
// }
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
// fn main() {
//     let coin = Coin::Quarter(UsState::Alabama);
//     println!("hey, {}", value_in_cents(coin));
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("lucky penny");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// } // TODO https://rust-kr.github.io/doc.rust-kr.org/ch06-02-match.html , 6.2장 하다 말았음 .

// Option<t> -------------------------------------------

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     println!("{:?}, {:?}", six, none);

//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => (), // 아무것도 안함
//     }

//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
// }
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(x) => Some(x + 1),
//     }
// }

// if let --------------------------------------------------------------------------
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("max: {}", max);
    }
}