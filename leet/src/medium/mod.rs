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

// # 12. Integer to Roman
// Link: https://leetcode.com/problems/integer-to-roman
fn int_to_roman(num: i32) -> String {
    const M: [&str; 4] = ["", "M", "MM", "MMM"];
    const C: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
    const X: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    const I: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];

    let mut result = String::new();
    result.push_str(M[(num / 1000) as usize]);
    result.push_str(C[((num % 1000) / 100) as usize]);
    result.push_str(X[((num % 100) / 10) as usize]);
    result.push_str(I[(num % 10) as usize]);
    result
}

// # 15. 3Sum
// Link: https://leetcode.com/problems/3sum
fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();
    let mut result: Vec<Vec<i32>> = Vec::with_capacity(nums.len() / 3);

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if sum > 0 {
                right -= 1;
            } else if sum < 0 {
                left += 1;
            } else {
                result.push(vec![nums[i], nums[left], nums[right]]);
                left += 1;

                while nums[left] == nums[left - 1] && left < right {
                    left += 1;
                }
            }
        }
    }
    result
}

// # 34. Find First and Last Position of Element in Sorted Array
// Link: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array
fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // if let Ok(index) = nums.binary_search(&target) {
    //     vec![
    //         nums.partition_point(|&i| i < target) as i32,
    //         nums.partition_point(|&i| i <= target) as i32 - 1,
    //     ]
    // } else {
    //     vec![-1, -1]
    // }

    let left = nums.partition_point(|&x| x < target);

    // INFO: if left is out of bounds or the value at left is not the target, return [-1, -1] since no such range exists
    if left == nums.len() || nums[left] != target {
        vec![-1, -1]
    } else {
        // INFO: looking for the right boundary in the remaining slice
        let right = left + nums[left..].partition_point(|&x| x <= target) - 1;
        vec![left as i32, right as i32]
    }
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

    #[test]
    fn test_int_to_roman() {
        let num = 3749;
        let result = int_to_roman(num);

        assert_eq!(result, format!("MMMDCCXLIX"));
    }

    #[test]
    fn test_three_sum() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let result = three_sum(nums);

        assert_eq!(result, vec![[-1, -1, 2], [-1, 0, 1]]);
    }

    #[test]
    fn test_search_range() {
        let result = search_range(vec![5, 7, 7, 8, 8, 10], 8);
        assert_eq!(result, vec![3, 4]);

        let result = search_range(vec![5, 7, 7, 8, 8, 10], 6);
        assert_eq!(result, vec![-1, -1]);
    }
}
