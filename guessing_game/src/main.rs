extern crate rand;

use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("quess the number");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("the secret number is {}", secret_number);

    loop {
        println!("\nplease input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read_line");

        println!("your guess number is: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please type a number");
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
