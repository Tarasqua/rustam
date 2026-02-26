pub fn largest_i32(list: &[i32]) -> Option<&i32> {
    if list.is_empty() {
        None
    } else {
        let mut largest = &list[0];
        for item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        Some(largest)
    }
}

pub fn largest_char(list: &[char]) -> Option<&char> {
    if list.is_empty() {
        None
    } else {
        let mut largest = &list[0];
        for item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        Some(largest)
    }
}

// PartialOrd trait is used to compare the items in the list
pub fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }

    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl Point<char, char> {
    fn distance_from_origin(&self) -> f32 {
        let x = self.x as u32 as f32;
        let y = self.y as u32 as f32;
        (x.powi(2) + y.powi(2)).sqrt()
    }
}

impl<X1: Clone, Y1> Point<X1, Y1> {
    // marking X1 as Clone to allow cloning of x in mixup method
    // and the same for Y2 in the mixup method to allow cloning of y from other point
    fn mixup<X2, Y2: Clone>(&self, other: &Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x.clone(),
            y: other.y.clone(),
        }
    }
}

pub fn pointer() {
    let point_1 = Point { x: 0.0, y: 1.0 }; // both x and y might be different types
    point_1.distance_from_origin();
    let point_2 = Point { x: '1', y: '2' };
    point_2.distance_from_origin();

    let point_3 = point_1.mixup(&point_2);
}
