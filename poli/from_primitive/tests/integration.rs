use from_primitive::FromPrimitive;

#[derive(Debug, PartialEq, FromPrimitive)]
enum Example {
    A,
    B,
    C,
    D,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive() {
        assert_eq!(Example::A as usize, 0);
        assert_eq!(Example::B as usize, 1);
        assert_eq!(Example::C as usize, 2);
        assert_eq!(Example::D as usize, 3);
        for (i, val) in (0..4).zip([Example::A, Example::B, Example::C, Example::D].into_iter()) {
            assert_eq!(Example::try_from(i), Ok(val));
        }
        assert!(Example::try_from(100).is_err());
    }
}
