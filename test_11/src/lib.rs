
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
pub fn add_two(num: usize) -> usize {
    num + 2
}
pub fn greeting(name: &str) -> String {
    format!("hello {name}")
}

#[cfg(test)]
mod tests {
    use super::*;
    // Using Assert equal we can determine that our add function is accurate
    // Purpose: demonstrate simple test functions
    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    // Purpose: Using panic we can see what constitutes as a failed test
    #[test]
    fn another(){
        // panic!("This test will fail");
    }
    // Using values that we know the expected outcome
    // we can test our can_hold function
    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle{
            width: 4,
            height: 5
        };
        let smaller = Rectangle{
            width: 2,
            height: 3
        };
        assert!(larger.can_hold(&smaller));
    }
    // Same as above but instead testing that the can_hold function
    // fails correctly, in this case returns false when a larger rect is
    // tested to fit in a smaller one
    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle{
            width: 4,
            height: 6
        };

        let smaller = Rectangle{
            width: 2,
            height: 3
        };
        assert!(!smaller.can_hold(&larger));
    }
    // using assert_eq to test a function with a known value
    #[test]
    fn it_adds_two(){
        let result = add_two(2);
        assert_eq!(result, 4);
    }
    // This is a test that doesn't test that greeting is correct(because it could change)
    // It only test that the parameter we pass it, appears in the message
    // If the name isn't present we output the greeting so we can debug
    #[test]
    fn greeting_contains_name(){
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was {result}"
        );
    }
    // We can test a function by checking its out of bounds states
    // in this case we know that a guess shouldn't be over 100 and therefore
    // it should panic, so the test only passes if it panics
    #[test]
    // Because should_panic can be imprecise(due to any panic okaying the test)
    // we can specify what the message should contain in this case "between 1 and 100"
    #[should_panic(expected = "between 1 and 100")]
    fn guess_greater_than_100(){
        Guess::new(200);
    }

    // The result type can be used for test
    // this makes it so we can use the '?' operator
    // The test will fail if the Err() type is returned and will
    // succeed if the Ok() type is returned
    #[test]
    fn it_works_results_test() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
