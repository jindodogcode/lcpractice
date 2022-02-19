//! # 1. Two Sum
//!
//! Given an array of integers `nums` and an integer `target`, return _indices of the two numbsers
//! such that they add up to_ `target`.
//!
//! You may assume that each input would have __exactly one solution__, and you may not sure the
//! _same_ element twice.
//!
//! You can return the answer in any order.
//!
//! ## Constraints:
//! - `2 <= nums.length <= 10⁴`
//! - `-10⁹ <= nums[i] <= 10⁹`
//! - `-10⁹ <= target <= 10⁹`
//! - __Only one valid answer exists.__
//!
//! ## Follow-up:
//! Can you come up with an algorithm that is less than `O(n²)` time complexity?

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut diff_index_map: HashMap<i32, i32> = HashMap::new();

    println!("target: {}", target);
    for (i, &n) in nums.iter().enumerate() {
        let diff = target - n;
        match diff_index_map.get(&diff) {
            Some(&index) => {
                return vec![index, i as i32];
            }
            None => {
                diff_index_map.insert(n, i as i32);
            }
        }
    }

    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(input, target);
        let expected = vec![0, 1];

        assert_eq!(expected, result);
    }

    #[test]
    fn example_two() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = two_sum(nums, target);
        let expected = vec![1, 2];

        assert_eq!(expected, result);
    }

    #[test]
    fn example_three() {
        let nums = vec![3, 3];
        let target = 6;
        let result = two_sum(nums, target);
        let expected = vec![0, 1];

        assert_eq!(expected, result);
    }
}
