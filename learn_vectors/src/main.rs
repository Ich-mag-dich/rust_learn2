// use core::fmt;
// use std::{any::Any, fmt::Display};

fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];

    // let mut v = Vec::new();

    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2]; //index ㅁ무ㄴ법
    // println!("third element is {}", third);

    // let third: Option<&i32> = v.get(2); //get method
    // match third {
    //     Some(third) => println!("third of element is {third}."),
    //     None => println!("there is no third element."),
    // }

    // for i in &v {
    //     println!("{i}");
    // }

    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    //     println!("{i}");
    // }

    // --------------------------

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Int(i) => println!("Int: {i}"),
            SpreadsheetCell::Float(i) => println!("Float: {i}"),
            SpreadsheetCell::Text(i) => println!("Text: {i}"),
        }
    }
}
