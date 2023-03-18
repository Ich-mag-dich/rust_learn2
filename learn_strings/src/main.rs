fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}, s2 is {s2}");

    s1 = String::from("lo");
    s1.push('l');
    println!("s1 is {s1}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    println!("{}", s1 + &s2); // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
}

// TODO 8.2장 부터 해야됨.
// https://rust-kr.github.io/doc.rust-kr.org/ch08-02-strings.html