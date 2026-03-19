struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_point() {
        let p = Point { x: 0, y: 7 };

        // NOTE: unpacking p to a and b
        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        // NOTE: unpacking p to x and y directly
        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);
    }

    #[test]
    fn test_destructure() {
        let p = Point { x: 0, y: 7 };

        let res = match p {
            Point { x, y: 0 } => format!("On the x axis at {x}"),
            Point { x: 0, y } => format!("On the y axis at {y}"),
            Point { x, y } => format!("On neither axis: ({x}, {y})"),
        };

        assert_eq!("On the y axis at 7", res);
    }

    #[test]
    fn test_complex() {
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

        assert_eq!(3, feet);
        assert_eq!(10, inches);
        assert_eq!(3, x);
        assert_eq!(-10, y);
    }
}
