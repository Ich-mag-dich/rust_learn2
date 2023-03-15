use std::io;

fn main() {
    println!("quess the number");
    println!("please input your guess");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read_line");

    println!("your guess number is: {}", guess);
}
