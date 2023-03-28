pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

fn internal_function() -> i32 {
    5
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn exploration() {
    //     assert_eq!(2 + 2, 4);
    // }

    // #[test]
    // fn larger_can_hold_smaller() {
    //     let larger = Rectangle {
    //         length: 8,
    //         width: 7,
    //     };
    //     let smaller = Rectangle {
    //         length: 5,
    //         width: 1,
    //     };

    //     assert!(larger.can_hold(&smaller));
    // }

    // #[test]
    // fn smaller_cannot_hold_larger() {
    //     let larger = Rectangle {
    //         length: 8,
    //         width: 7,
    //     };
    //     let smaller = Rectangle {
    //         length: 5,
    //         width: 1,
    //     };

    //     assert!(!smaller.can_hold(&larger));
    // }
    #[test]
    #[ignore]
    fn it_adds() {
        assert_eq!(4, add(2, 2));
    }
    #[test]
    #[ignore]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    #[test]
    fn internal() {
        assert_eq!(5, internal_function());
    }
}
