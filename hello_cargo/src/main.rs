fn main() {
    println!("Hello, world!");
    println!("{}", hello_cargo("hello cargo!!"));
}

fn hello_cargo(text: &str) -> String {
    text.to_uppercase()
}
