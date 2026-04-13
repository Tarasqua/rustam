use std::num::NonZeroU32;

fn main() {}

pub enum Shape {
    Square(Square),
    Circle(Circle),
}
pub struct Circle(NonZeroU32);
pub struct Square(NonZeroU32);

impl Circle {
    pub fn from_radius(r: NonZeroU32) -> Self {
        Self(r)
    }
    pub fn radius(&self) -> NonZeroU32 {
        self.0
    }
}

impl Square {
    pub fn from_side(s: NonZeroU32) -> Self {
        Self(s)
    }
    pub fn side(&self) -> NonZeroU32 {
        self.0
    }
}

// Convenience fallible API
impl Shape {
    pub fn radius(&self) -> Result<NonZeroU32, &'static str> {
        match self {
            Shape::Square(_) => Err("Only circles have a radius"),
            Shape::Circle(c) => Ok(c.radius()),
        }
    }
    pub fn side(&self) -> Result<NonZeroU32, &'static str> {
        match self {
            Shape::Square(s) => Ok(s.side()),
            Shape::Circle(_) => Err("Only squares have sides"),
        }
    }
    pub fn is_circle(&self) -> bool {
        matches!(self, Shape::Circle(_))
    }
    pub fn is_square(&self) -> bool {
        matches!(self, Shape::Square(_))
    }
}

// ==========

struct Account {
    // Must be alphabetical and less than 8 characters long.
    username: Username,
    // Must be alphanumeric and between 8 and 16 characters.
    password: Password,
}

struct Password(String);
struct Username(String);

impl Password {
    fn new(raw: &str) -> Option<Self> {
        (raw.len() > 8 && raw.len() < 16 && raw.chars().all(char::is_alphanumeric))
            .then_some(Self(raw.to_owned()))
    }
    fn get(&self) -> &str {
        &self.0
    }
}

impl Username {
    fn new(raw: &str) -> Option<Self> {
        (raw.len() < 8 && raw.chars().all(char::is_alphabetic)).then_some(Self(raw.to_owned()))
    }
    fn get(&self) -> &str {
        &self.0
    }
}
