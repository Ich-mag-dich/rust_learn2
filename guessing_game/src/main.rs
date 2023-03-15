extern crate rand;

use rand::Rng;
use std::{ cmp::Ordering, io };

fn main() {
    println!("guess the number");
    println!("if you want to exit this game, input 'quit'.");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("the secret number is {secret_number}");

    loop {
        println!("\nplease input your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("!!! failed to read_line");

        // 어떠한 변수의 값을 출력할때라면 중괄호 안에 바로 집어넣고 ex) {guess}
        // 어떠한 표현식의 결과값을 출력할때에는 빈 중괄호와 문자열 뒤 콤마로 출력. ex) "{}", guess + 2

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                println!("your guess number is: {guess}");
                num
            }
            Err(_) => {
                if guess.trim() == "quit".trim() {
                    break;
                }
                println!("!!! please type a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Equal");
                break;
            }
        }
    }
}