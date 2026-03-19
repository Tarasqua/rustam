// INFO: usage of the `at` operator @
// operator @ lets us create a variable that holds a value at the same time we’re testing that value for a pattern match

enum Message {
    Hello { id: i32 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() {
        let msg = Message::Hello { id: 5 };

        let result = match msg {
            // INFO: id is bound to the value 5, but we can still use it in the format! macro
            Message::Hello { id: id @ 3..=7 } => format!("Found an id in range: {id}"),
            Message::Hello { id: 10..=12 } => "Found an id in another range".to_string(),
            Message::Hello { id } => format!("Found some other id: {id}"),
        };

        assert_eq!(result, "Found an id in range: 5");
    }
}
