// fn main() {
//     let mut s = "Hello";

//     println!("{s}"); // hello
//     {
//         println!("{s}"); //hello
//         s = "world1111";
//         println!("{s}"); // world
//         let mut s = "scope";
//         println!("{s}"); // scope
//         s = "here";
//         println!("{s}"); // here
//     }
//     println!("{s}"); // world
// }

//

// fn main() {
//     let mut s = String::from("Hello");
//     s.push_str(", world!");
//     println!("{s}");
// }

// fn main() {
//     let x = 5;
//     let y = x;
//     // x and y는 각각 5라는 값을 가지고 변수가 스택에 push.
// }

// fn main() {
//     let s1 = String::from("Hello");
//     let s2 = s1.clone(); // heap 데이터까지 복사
//     println!("s1: {s1}, s2: {s2}");
// }

// borrow  : & character
// fn main() {
//     let s1 = String::from("Hello");
//     println!("qwe {:?}", another(&s1));
//     println!("{s1}")
// }

// fn another(ss: &String) -> String {
//     return format!("another, {}", ss.as_str());
// }

fn main() {
    let s = String::from("Hello world my friend");
    println!("1st word is: {}", slice_word(&s[..], 1)); // hello
    println!("2nd word is: {}", slice_word(&s[..], 2)); // world
    println!("3rd word is: {}", slice_word(&s[..], 3)); // my
    println!("4th word is: {}", slice_word(&s[..], 4)); // friend
}
fn slice_word(s: &str, number: i32) -> &str {
    let bytes: &[u8] = s.as_bytes();
    let mut count: i32 = 0;
    let mut _spec_index: usize = 0;
    let mut _spec_index2: usize = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            count = count + 1;
            _spec_index = _spec_index2;
            _spec_index2 = i;
            if count == number {
                return &s[_spec_index.._spec_index2].trim();
            }
        }
    }
    return &s[_spec_index2 + 1..s.len()].trim();
}
// output:
// 1st word is: Hello
// 2nd word is: world
// 3rd word is: my
// 4th word is: friend