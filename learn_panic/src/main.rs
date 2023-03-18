use std::fs::{ self, File };
use std::io::{ self, Read };

fn main() {
    // panic!("crash and burn.");

    // let greeting_file_result = File::open("hello.txt");
    //
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) =>
    //         match error.kind() {
    //             ErrorKind::NotFound =>
    //                 match File::create("hello.txt") {
    //                     Ok(file) => file,
    //                     Err(error) =>
    //                         panic!(
    //                             "problem creating the file, {:?}",
    //                             error
    //                         ),
    //                 }
    //             other_error => {
    //                 panic!("problem opening file: {:?}", other_error);
    //             }
    //         }
    // };

    // let mut greeting_file = File::open("hello.txt").unwrap_or_else(
    //     |error| {
    //         if error.kind() == ErrorKind::NotFound {
    //             File::create("hello.txt").unwrap_or_else(|error| {
    //                 panic!("problem creating file {:?}", error);
    //             })
    //         } else {
    //             panic!("problem opening file {:?}", error);
    //         }
    //     }
    // );

    // let greeting_file = File::open("hello.txt").unwrap();

    // let greeting_file = File::open("hello.txt").expect(
    //     "hello.txt should be include in this project"
    // );

    //

    let filename = String::from("hello.txt");
    println!("{}", read_from_file(&filename).unwrap());
}

// fn read_from_file(filename: &String) -> Result<String, io::Error> {
//     let read_file_result = File::open(&filename);
//     let mut read_file = match read_file_result {
//         Ok(file) => file,
//         Err(error) => {
//             return Err(error);
//         }
//     };
//     let mut contents = String::new();
//     match read_file.read_to_string(&mut contents) {
//         Ok(_) => Ok(contents),
//         Err(error) => Err(error),
//     }
// }

//  위의 함수를 ? 를 이용해 간결하게 표현하는 모습,,

// fn read_from_file(filename: &String) -> Result<String, io::Error> {
//     let mut read_file = File::open(&filename)?;
//     let mut contents = String::new();
//     read_file.read_to_string(&mut contents)?;
//     Ok(contents)
// }

// 위의 함수를 더 간결하게 표현한 모습,,

// fn read_from_file(filename: &String) -> Result<String, io::Error> {
//     let mut contents = String::new();
//     File::open(&filename)?.read_to_string(&mut contents)?;
//     Ok(contents)
// }

// 위의 함수를 더 간결하게 표현한 모습,.,

fn read_from_file(filename: &String) -> Result<String, io::Error> {
    fs::read_to_string(&filename)
}