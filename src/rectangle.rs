#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // fn new(width: u32, height: u32) -> Self {
    //     Self { width, height }
    // }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn print_dimensions(&self) {
        println!("{:#?}", self)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

pub fn run() {
    let rect = Rectangle {
        width: 10,
        height: 5,
    };
    let area = rect.area();
    rect.print_dimensions();
    dbg!(&area);
    dbg!(&rect);

    let other_rect = Rectangle { width: 5, ..rect };
    let can_hold = rect.can_hold(&other_rect);
    dbg!(&can_hold);

    let square = Rectangle::square(5);
    dbg!(&square);
}
