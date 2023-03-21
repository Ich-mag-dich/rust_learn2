use std::cmp::PartialOrd;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

fn largest<'a, T>(list: &'a [T]) -> &'a T
where
    T: PartialOrd,
{
    let mut largest_num = &list[0];
    for number in list {
        if number > largest_num {
            largest_num = number;
        }
    }
    largest_num
}
