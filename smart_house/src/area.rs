trait Area {
    fn area(&self) -> f32;
}

struct Rectangle {
    width: f32,
    height: f32,
}

impl Area for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

struct Circle {
    radius: f32,
}

impl Area for Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius.powi(2)
    }
}

struct RightTriangle {
    base: f32,
    height: f32,
}

impl Area for RightTriangle {
    fn area(&self) -> f32 {
        (self.base * self.height) / 2.0
    }
}
