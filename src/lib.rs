// pub fn add(left:usize,right:usize)->usize{
//     left+right
// }

// #[cfg(test)]
// mod tests{
//     use super::*;

//     #[test]
//     fn exploration(){
//         let result=add(2, 4);
//         assert_eq!(result,6);
//     }

//     #[test]
//     fn another(){
//         panic!("Make this test fail");
//     }
// }
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
pub fn greeting(name: &str) -> String {
    format!("Hello! {name}")
}
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be grater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }

        Guess { value }
    }
}
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass(){
        let value= prints_and_returns_10(4);
        assert_eq!(10,value);
    }
    #[test]
    #[ignore = "my choice"]
    fn this_test_will_pass_too(){
        let value= prints_and_returns_10(8);
        assert_eq!(10,value);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 20,
        };
        let smaller = Rectangle {
            width: 5,
            height: 4,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle {
            width: 10,
            height: 20,
        };
        let smaller = Rectangle {
            width: 5,
            height: 4,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Naveen");
        assert!(
            result.contains("Naveen"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }
    #[test]
    #[should_panic = "less than or equal to 100"]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
