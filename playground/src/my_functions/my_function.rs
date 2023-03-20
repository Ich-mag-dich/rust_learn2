use std::{fs, io};

pub fn my_print(s: &str) {
    println!("my print {}", &s);
}

pub fn text_read(filename: &String) -> Result<String, io::Error> {
    fs::read_to_string(&filename)
}
