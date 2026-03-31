use std::{
    fmt::{Debug, Display},
    ops::Add,
};

pub struct Point<T> {
    x: T,
    y: T,
}

// INFO: явно указываем, что T должен быть тем типом, который реализует сложение Add<Output = T>
impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

trait MyTrait {}

impl<T> MyTrait for T where T: Debug + Display {}
