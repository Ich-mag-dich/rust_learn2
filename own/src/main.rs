fn main() {
    let mut s = "Hello";

    println!("{s}"); // hello
    {
        println!("{s}"); //hello
        s = "world";
        println!("{s}"); // world
        let mut s = "scope";
        println!("{s}"); // scope
        s = "here";
        println!("{s}"); // here
    }
    println!("{s}"); // world
}

// TODO  4.1 부터
