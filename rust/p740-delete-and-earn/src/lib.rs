//! # 740. Delete and Earn
//!
//! You are given an integer array `nums`. You want to maximize the number of points you get by
//! performing the following operations any number of times:
//! - Pick any `nums[i]` and delete it to earn `nums[i]` points. Afterwards, you must delete
//! **every** element equal to `nums[i] - 1` and **every** element equal to `nums[i] + 1`.
//!
//! *Return the **maximum number of points** you can earn by applying the above operation some
//! number of times*.
//!
//! ## Constraints:
//!
//! - `1 <= nums.length <= 2 * 10⁴`
//! - `1 <= nums[i] <= 10⁴`

struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut points = BTreeMap::new();

        for n in nums {
            points.entry(n).and_modify(|v| *v += n).or_insert(n);
        }

        // (num, point value)
        let mut one_back = (0, 0);
        let mut two_back = (0, 0);

        for p in points {
            let temp = one_back;
            let (num, value) = p;

            if num - 1 == one_back.0 {
                one_back = (num, i32::max(value + two_back.1, one_back.1));
            } else {
                one_back = (num, value + one_back.1);
            }

            two_back = temp;
        }

        one_back.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = vec![3, 4, 2];
        let expected = 6;

        run_test(input, expected);
    }

    #[test]
    fn example_two() {
        let input = vec![2, 2, 3, 3, 3, 4];
        let expected = 9;

        run_test(input, expected);
    }

    fn run_test(input: Vec<i32>, expected: i32) {
        let result = Solution::delete_and_earn(input);

        assert_eq!(expected, result);
    }
}
