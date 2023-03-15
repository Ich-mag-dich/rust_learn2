fn main() {
    println!("Hello, world!");
    println!("this is {}", another_function());
}

fn another_function() -> String {
    return String::from("another_function");
}
