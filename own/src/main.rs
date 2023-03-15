fn main() {
    let s = "Hello";

    println!("{s}");
    {
        let s = "scope";
        println!("{s}");
    }
    println!("{s}");
}
