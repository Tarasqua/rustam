trait Draw {
    fn draw(&self);
}

struct Circle;

impl Circle {
    fn new() -> Self {
        Circle
    }
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a circle");
    }
}

struct Square;

impl Square {
    fn new() -> Self {
        Square
    }
}

impl Draw for Square {
    fn draw(&self) {
        println!("Drawing a square");
    }
}

pub fn make_figures() {
    let figures: Vec<Box<dyn Draw>> = vec![Box::new(Circle), Box::new(Square)];
    for figure in figures {
        figure.draw();
    }
}
