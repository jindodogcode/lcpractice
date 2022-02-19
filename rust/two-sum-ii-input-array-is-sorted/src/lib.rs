//! # 167. Two Sum II - Input array is sorted
//!
//! Given a **1-indexed** array of integers `numbers` that is already **sorted in non-decreasing
//! order**, find two numbers such that they add up to a specific `target` number. Let these two
//! numbers be `numbers[index₁]` and `numbers[index₂]` where `1 <= first < second <=
//! numbers.length`.
//!
//! Return _the indices of the two numbers, `index₁` and `index₂`, as _an integer array_ `[index₁,
//! index₂]` _of length 2_.
//!
//! The tests are generated such that there is **exactly one solution**. You **may not** use the
//! same element twice.
//!
//! ** Constraints:
//! - `2 <= numbers.length <= 3 * 10⁴`
//! - `-1000 <= numbers[i] <= 1000`
//! - `numbers` is sorted in **non-decreasing order**.
//! - -1000 <= target <= 1000`
//! - The test are generated such that there is **exactly one solution**.
use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            let sum = numbers[left] + numbers[right];

            match sum.cmp(&target) {
                Ordering::Less => {
                    left += 1;
                }
                Ordering::Greater => {
                    right -= 1;
                }
                Ordering::Equal => {
                    return vec![left as i32 + 1, right as i32 + 1];
                }
            }
        }

        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(numbers, target);
        let expected = vec![1, 2];

        assert_eq!(expected, result);
    }

    #[test]
    fn example_two() {
        let numbers = vec![2, 3, 4];
        let target = 6;
        let result = Solution::two_sum(numbers, target);
        let expected = vec![1, 3];

        assert_eq!(expected, result);
    }

    #[test]
    fn example_three() {
        let numbers = vec![-1, 0];
        let target = -1;
        let result = Solution::two_sum(numbers, target);
        let expected = vec![1, 2];

        assert_eq!(expected, result);
    }
}
