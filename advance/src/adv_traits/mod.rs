use std::fmt;
use std::ops::Add;

pub trait Iterator {
    type Item; // INFO: Item is the type of the value returned by next

    fn next(&mut self) -> Option<Self::Item>;
}

// =======================================

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Point; // INFO: default output type is Point

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// =======================================

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

// INFO: To add Millimeters and Meters, we specify impl Add<Meters> to set the value of the Rhs (right hand side - Meters) type parameter instead of using the default of Self.
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// =======================================

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

pub fn human() {
    let person = Human;
    person.fly(); // *waving arms furiously*
    Pilot::fly(&person); // This is your captain speaking.
    Wizard::fly(&person); // Up!
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

pub fn animal() {
    println!("A baby dog is called a {}", Dog::baby_name()); // Spot
    // Animal::baby_name() // ERROR: Rust doesn’t know which implementation to use (Animal::baby_name doesn’t have a self parameter)
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // puppy
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
}

// =======================================

// INFO: OutlinePrint requires the Display to extend functionality (to use to_string)
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn outline_point() {
    let point = Point::new(1, 2);
    point.outline_print();
}

// =======================================

// INFO: External Traits with the Newtype Pattern
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn wrap() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_add() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(3, 4);
        let p3 = p1 + p2;
        assert_eq!(p3, Point::new(4, 6));
    }

    #[test]
    fn test_meters_add() {
        let meters = Meters(1);
        let millimeters = Millimeters(200);
        let result = millimeters + meters;
        assert_eq!(result, Millimeters(1200));
    }
}
