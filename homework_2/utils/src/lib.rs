#![allow(dead_code)]

pub mod logging_;
pub mod profiling_;
pub mod tracing_;

pub fn fast_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_search() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(fast_search(&arr, 3), Some(2));
        assert_eq!(fast_search(&arr, 6), None);
        assert_eq!(fast_search(&arr, 1), Some(0));
        assert_eq!(fast_search(&arr, 5), Some(4));
    }
}
