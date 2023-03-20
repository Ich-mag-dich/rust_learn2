fn main() {
    println!("Hello, world!");
    println!("{}", hello_emacs());
}

fn hello_emacs() -> String {
    String::from("hello emacs!")
}