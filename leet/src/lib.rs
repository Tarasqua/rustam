use std::{
    collections::{HashMap, HashSet},
    i32,
};

/// # 1. Two Sum
/// Link: https://leetcode.com/problems/two-sum
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: HashMap<&i32, usize> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        if let Some(&index) = result.get(&(target - num)) {
            return vec![index as i32, i as i32];
        }
        result.insert(num, i);
    }
    vec![]
}

/// # 1748. Sum of Unique Elements
/// Link: https://leetcode.com/problems/sum-of-unique-elements
pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    let mut count = HashMap::with_capacity(nums.len());
    for num in nums {
        *count.entry(num).or_insert(0) += 1; // dereference the mutable reference to increment the count
    }
    count
        .into_iter() // convert the HashMap into an iterator of (number, count) pairs
        .filter(|&(_, count)| count == 1) // filter out number that appear more than once
        .map(|(num, _)| num) // map to the number itself
        .sum() // sum the unique numbers
}

/// # 1200. Minimum Absolute Difference
/// Link: https://leetcode.com/problems/minimum-absolute-difference
pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut arr = arr; // move arr to mutable variable
    arr.sort_unstable();
    let mut min_diff = i32::MAX;
    let mut result: Vec<Vec<i32>> = Vec::new();

    for i in 1..arr.len() {
        let diff = arr[i] - arr[i - 1];
        if diff < min_diff {
            min_diff = diff;
            result.clear();
            result.push(vec![arr[i - 1], arr[i]]);
        } else if diff == min_diff {
            result.push(vec![arr[i - 1], arr[i]]);
        }
    }
    result
}

/// # 929. Unique Email Addresses
/// Link: https://leetcode.com/problems/unique-email-addresses
pub fn num_unique_emails(emails: Vec<String>) -> i32 {
    let mut unique_emails = HashSet::new();
    for email in emails {
        let parts: Vec<&str> = email.split('@').collect();
        let local = parts[0].split('+').next().unwrap().replace('.', "");
        let normalized_email = format!("{}@{}", local, parts[1]);
        unique_emails.insert(normalized_email);
    }
    unique_emails.len() as i32
}

/// # 1614. Maximum Nesting Depth of the Parentheses
/// Link: https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses
pub fn max_depth(s: String) -> i32 {
    let mut max_depth = 0;
    let mut current_depth = 0;
    for c in s.chars() {
        if c == '(' {
            current_depth += 1;
            max_depth = max_depth.max(current_depth);
        } else if c == ')' {
            current_depth -= 1;
        }
    }
    max_depth
}

/// # 1480. Running Sum of 1d Array
/// Link: https://leetcode.com/problems/running-sum-of-1d-array
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    // let mut result = Vec::with_capacity(nums.len());
    // let mut sum = 0;
    // for num in nums {
    //     sum += num;
    //     result.push(sum);
    // }
    // result

    // INFO: scan is an iterator adapter that takes an initial state and a closure, and produces an iterator that yields the result of applying the closure to each item in the original iterator, while also updating the state.
    // 0 - initial state, acc - mutable reference to the state, x - current item from the original iterator.
    // Some(*sum) since scan expects an Option to be returned, we return Some with the current sum, and update the acc with the new sum. *sum dereferences the mutable reference to get the current value of sum, and then we add x to it and update acc with the new value.
    nums.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect()

    // nums.iter_mut().fold(0, |acc, x| {
    //         *x += acc;
    //         *x
    //     });
    //     nums
}

/// # 1394. Find Lucky Integer in an Array
/// Link: https://leetcode.com/problems/find-lucky-integer-in-an-array
pub fn find_lucky(arr: Vec<i32>) -> i32 {
    // let mut count = HashMap::with_capacity(arr.len());
    // for num in arr {
    //     *count.entry(num).or_insert(0) += 1;
    // }
    // count
    //     .into_iter()
    //     .filter(|&(num, count)| num == count)
    //     .map(|(num, _)| num)
    //     .max()
    //     .unwrap_or(-1)

    let mut count = vec![0u16; 501]; // 0..501 since the problem states that the numbers in the array are between 1 and 500
    for num in arr {
        count[num as usize] += 1;
    }
    // reverse the range to find the largest lucky integer
    for i in (1..=500).rev() {
        if count[i] == i as u16 {
            return i as i32;
        }
    }
    -1
}

// # 12. Integer to Roman
// Link: https://leetcode.com/problems/integer-to-roman
pub fn int_to_roman(num: i32) -> String {
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

// # 349. Intersection of Two Arrays
// Link: https://leetcode.com/problems/intersection-of-two-arrays
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // INFO: solution 1
    // let mut nums1_set: HashSet<i32> = HashSet::from_iter(nums1);
    // let mut result: Vec<i32> = Vec::new();

    // for num in nums2 {
    //     if nums1_set.remove(&num) {z
    //         result.push(num);
    //     }
    // }
    // result

    // INFO: solution 2 (most idiomatic)
    let mut set: HashSet<_> = nums1.into_iter().collect();
    nums2.into_iter().filter(|n| set.remove(n)).collect()

    // INFO: solution 3
    // let set1: HashSet<_> = nums1.into_iter().collect();
    // let set2: HashSet<_> = nums2.into_iter().collect();

    // // Method intersection returns links -> .copied() is required
    // set1.intersection(&set2).copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums, target);

        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_sum_of_unique() {
        let nums = vec![1, 2, 3, 2];
        let result = sum_of_unique(nums);

        assert_eq!(result, 4);
    }

    #[test]
    fn test_minimum_abs_difference() {
        let arr = vec![4, 2, 1, 3];
        let result = minimum_abs_difference(arr);

        assert_eq!(result, vec![[1, 2], [2, 3,], [3, 4]]);
    }

    #[test]
    fn test_num_unique_emails() {
        let emails = vec![
            "test.email+alex@leetcode.com".to_string(),
            "test.e.mail+bob.cathy@leetcode.com".to_string(),
            "testemail+david@lee.tcode.com".to_string(),
        ];
        let result = num_unique_emails(emails);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_max_depth() {
        let s = "()(())((()()))".to_string();
        let result = max_depth(s);

        assert_eq!(result, 3);
    }

    #[test]
    fn test_running_sum() {
        let nums = vec![1, 2, 3, 4];
        let result = running_sum(nums);

        assert_eq!(result, vec![1, 3, 6, 10]);
    }

    #[test]
    fn test_find_lucky() {
        let arr = vec![2, 2, 3, 4];
        let result = find_lucky(arr);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_int_to_roman() {
        let num = 3749;
        let result = int_to_roman(num);

        assert_eq!(result, format!("MMMDCCXLIX"));
    }

    #[test]
    fn test_intersection() {
        let nums1 = [1, 2, 2, 1].to_vec();
        let nums2 = vec![2, 2];
        let result = intersection(nums1, nums2);

        assert_eq!(result, vec![2]);
    }
}
