#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            height: 7,
            width: 8,
        };
        let smaller = Rectangle {
            height: 1,
            width: 5,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            height: 7,
            width: 8,
        };
        let smaller = Rectangle {
            height: 1,
            width: 5,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greetings("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        return if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Two plust two does not equal four"))
        };
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        return other.width < self.width && other.height < self.height;
    }
}

pub fn add_two(a: i32) -> i32 {
    return a + 2;
}

pub fn greetings(name: &str) -> String {
    return format!("Hello: {}", name);
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!(
                "Invalid value, must be between 1 and 100 and it was {}",
                value
            );
        }
        return Guess { value };
    }
}
