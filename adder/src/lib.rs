use std::cmp::Ordering;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

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

fn add_two(a: u64) -> u64 {
    a + 2
}

pub fn add_two_pub(a: u64) -> u64 {
    a + 2
}

// struct User {
//     id: u32,
//     name: String,
//     uuid: String, // We want to ignore this in comparisons
// }

// impl PartialEq for User {
//     fn eq(&self, other: &Self) -> bool {
//         self.id == other.id && self.name == other.name // ignore uuid
//     }
// }

// impl PartialOrd for User {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         // Compare IDs first
//         self.id.partial_cmp(&other.id).and_then(|ord| {
//             if ord == Equal {
//                 // If IDs are equal, compare names
//                 self.name.partial_cmp(&other.name)
//             } else {
//                 Some(ord)
//             }
//         })
//     }
// }

// impl Debug for User {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         f.debug_struct("User")
//             .field("id", &self.id)
//             .field("name", &self.name)
//             // .field("uuid", &self.uuid) // Simply omit this line
//             .finish()
//     }
// }

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

pub struct Guess {
    value: i32,
}

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {value}.");
//         }

//         Guess { value }
//     }
// }

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }

        Guess { value }
    }
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
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
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
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        )
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
