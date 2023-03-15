use std::process::exit;

fn main() {
    println!("the number is {}", another_function());
}

fn another_function() -> i32 {
    println!("input integer:");
    let mut input_num = String::new();
    std::io
        ::stdin()
        .read_line(&mut input_num)
        .expect("cannot read line");
    let input_num: i32 = match input_num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("this is not integer: {input_num}");
            exit(0)
        }
    };
    return input_num;
}