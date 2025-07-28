/*
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width) && (self.height > other.height)
    }
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

pub fn geeting(name: &str) -> String {
    format!("Hello {name}!")
}

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }
        Guess { value }
    }
}
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}"); // This is only printed if fails by default
    10
}





// ****************************** //
// ********* Tests ************** //
// ****************************** //

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger: Rectangle = Rectangle {
            width: 8,
            height: 7
        };
        let smaller: Rectangle = Rectangle {
            width: 5,
            height: 1
        };
        
        assert!(larger.can_hold(&smaller));
        //Asser expects a boolean. If alse, asserts        
    }
    
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger: Rectangle = Rectangle {
            width: 8,
            height: 7
        };
        let smaller: Rectangle = Rectangle {
            width: 5,
            height: 1
        };
        
        assert!(!smaller.can_hold(&larger));
        //Asser expects a boolen. If alse, asserts        
    }
    
    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
        //assert_eq checks if return value equal to the second paramater
        //If fails, gives extra info
    }
    
    //assert_eq, assert_ne use == and !=, and debug print
    // so PartialEq and Debug traits needs to be implemnted
    
    #[test]
    fn greeting_contains_name() {
        let result: String  = geeting("Carol");
        assert!(result.contains("Carol"));
    }
    
    #[test]
    #[should_panic] // Indicating that we are expecting panic here
    fn greater_than_100() {
        Guess::new(200);
    } // Problem is that this passes if panics for any reason, not too precise
    
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100_precise() {
        Guess::new(101);
    } // New we are checking if it panics with this exact panic message
    
    //Using Result<T, E> in tests
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }
    
    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }
    
    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }
    
    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }
    
    #[test]
    #[ignore] // ignoring it by defult
    fn expensive_test() { //this is really long for example
    }
   
}

//Commands to try:
//> cargo test -- --test-threads=1
//    useful when tests can interfere (file reads / writes for example)
//> cargo test -- --show-output
//    test prints are shown even when succeeds
//> cargo test one_hundred
//    test only one particaular test function
//> cargo test add
//    all test will run that contains add
//> cargo test -- --ignored
//    run only ignored tests
//> cargo test -- --include-ignored
//    runs all tests, including ignored ones
