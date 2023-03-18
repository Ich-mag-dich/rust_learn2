mod my_function;
use std::{ fs, io };

pub use my_function::my_print;
fn main() {
    my_print::my_print("qweqwe");
    let s = read_file(&String::from("hello.txt"));
    match s {
        Ok(str) => println!("{str}"),
        Err(e) => panic!("{}", e),
    }
}

fn read_file(s: &String) -> Result<String, io::Error> {
    fs::read_to_string(&s)
}