// //
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = build_user(
//         String::from("someone@example.com"),
//         String::from("someusername123"),
//     );
//     let user2 = User {
//         active: user1.active,
//         username: (&user1.username).to_string(),
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count,
//     };
//     let user3: User = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

// ----------------------------------

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn round(&self) -> u32 {
        (self.width + self.height) * 2
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };
    let rect3 = Rectangle::square(3);
    println!("area of the rectangle is {} square pixels.", rect1.area());
    println!("round of the rectangle is {}.", rect1.round());
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect3 width: {}, height: {}", rect3.width, rect3.height);
}
