//! # 198. House Robber
//!
//! You Are a professional robber planning to rob houses along a street. Each house has a certain
//! amount of money stashed, the only constraint stopping you from robbing each of them is that
//! adjacent houses have security system connected and **it will automatically contact the polic if
//! two adjacent houses were broken into on the same night**.
//!
//! Given an integer array `nums` representing the amount of money of each house, return *the
//! maximum amount of money you can rob tonight* **without alerting the police**.
//!
//! ## Constraints:
//! - `1 <= nums.length <= 100`
//! - `0 <- nums[i] <= 400`

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        if n == 1 {
            return nums[0];
        }

        let mut value = vec![0; n + 1];
        value[n] = 0;
        value[n - 1] = nums[n - 1];

        for i in (0..=(n - 2)).rev() {
            value[i] = i32::max(value[i + 1], value[i + 2] + nums[i]);
        }

        value[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = vec![1, 2, 3, 1];
        let expected = 4;

        run_test(input, expected);
    }

    #[test]
    fn example_two() {
        let input = vec![2, 7, 9, 3, 1];
        let expected = 12;

        run_test(input, expected);
    }

    fn run_test(input: Vec<i32>, expected: i32) {
        let result = Solution::rob(input);

        assert_eq!(result, expected);
    }
}
