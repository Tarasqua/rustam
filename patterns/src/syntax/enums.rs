enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message_ {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color), // NOTE: nested enum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_destructure() {
        let msg = Message::ChangeColor(0, 160, 255);

        let res = match msg {
            Message::Quit => "Quit".to_string(),
            Message::Move { x, y } => format!("Move: ({x}, {y})"),
            Message::Write(s) => format!("Write: {s}"),
            Message::ChangeColor(r, g, b) => format!("ChangeColor: ({r}, {g}, {b})"),
        };

        assert_eq!("ChangeColor: (0, 160, 255)", res);
    }

    #[test]
    fn test_destructure_nested() {
        let msg = Message_::ChangeColor(Color::Rgb(0, 160, 255));

        let res = match msg {
            Message_::Quit => "Quit".to_string(),
            Message_::Move { x, y } => format!("Move: ({x}, {y})"),
            Message_::Write(s) => format!("Write: {s}"),
            Message_::ChangeColor(Color::Rgb(r, g, b)) => format!("ChangeColor: ({r}, {g}, {b})"),
            Message_::ChangeColor(Color::Hsv(h, s, v)) => format!("ChangeColor: ({h}, {s}, {v})"),
        };

        assert_eq!("ChangeColor: (0, 160, 255)", res);
    }
}
