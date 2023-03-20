mod my_functions;

pub use my_functions::my_function;
fn main() {
    my_function::my_print("qwe qwe");
    let file_name = String::from("hello.txt");
    let contents = my_function::text_read(&file_name).expect("can't read file");
    println!("contents: {{\n{}\n}}", contents);
}
