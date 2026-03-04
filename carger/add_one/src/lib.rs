pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_one_test() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
