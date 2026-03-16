/// # 7. Reverse Integer
/// Link: https://leetcode.com/problems/reverse-integer
pub fn reverse(x: i32) -> i32 {
    let mut res: i32 = 0;
    let mut cur: i32 = x;

    while cur != 0 {
        // // INFO: checked_mul computes the product and checks for overflow making a "blank" for processing
        // match res.checked_mul(10) {
        //     None => return 0,
        //     // INFO: checked_add computes the sum and checks for overflow making a "blank"
        //     Some(tmp) => match tmp.checked_add(cur % 10) {
        //         None => return 0,
        //         Some(r) => res = r,
        //     },
        // }
        // cur /= 10;

        // INFO: another way to write the above
        if let Some(new_res) = res
            .checked_mul(10)
            .and_then(|tmp| tmp.checked_add(cur % 10))
        {
            res = new_res;
        } else {
            return 0;
        }
        cur /= 10;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(120), 21);
    }
}
