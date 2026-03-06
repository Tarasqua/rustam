fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`.
        // word = optional_target {
        //     assert_eq!(word, target);
        // }
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.
        // integer = optional_integers.pop() {
        //     assert_eq!(integer, cursor);
        //     cursor -= 1;
        // }
        // Some(Some(integer)) since optional_integers contains Option<i8> values and .pop returns Option<T> -> Option<Option<i8>> -> Some(Some(integer))
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        // equivalent to the while-let statement above
        // loop {
        //     match optional_integers.pop() {
        //         // If we get Some(Some(value)), the loop continues
        //         Some(Some(integer)) => {
        //             assert_eq!(integer, cursor);
        //             cursor -= 1;
        //         }
        //         // If we get Some(None) or just None, we stop
        //         _ => break,
        //     }
        // }

        assert_eq!(cursor, 0);
    }
}
