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
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        return Err("division by zero");
    }
    Ok(a / b)
}

#[cfg(test)]
mod tests {
    // Use this line to inherit from the lib scope, as mod tests is a self-contained scope.
    // this is called inner/outer module.
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn how_to_fail_a_test() {
        panic!("Make this test fail");
    }

    #[test]
    fn test_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 6,
        };
        let smaller = Rectangle {
            width: 5,
            height: 2,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[should_panic]
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

    #[test]
    #[ignore]
    fn test_divide() -> Result<(), &'static str> {
        assert_eq!(divide(10, 2)?, 5); // question mark operator used here
        assert_eq!(divide(10, 0)?, 5); // this assertion will fail and return Err("division by zero")
        Ok(())
    }
}
