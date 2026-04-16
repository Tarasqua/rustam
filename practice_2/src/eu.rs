#![allow(dead_code)]

enum Mm {}
enum Cm {}

#[cfg(test)]
mod tests {
    use euclid::Length;

    use super::*;

    #[test]
    fn test_add() {
        let length1: Length<u8, Mm> = Length::new(250);
        let length2: Length<u8, Mm> = Length::new(3);
        let _length3: Length<u8, Cm> = Length::new(3);

        // let sum = length1 + length3; // ERROR: cannot add `Length<u8, Mm>` to `Length<u8, Cm>`
        assert_eq!((length1 + length2).get(), 253);
    }
}
