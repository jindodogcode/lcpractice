//! # 746. Min Cost Climbing Stairs
//!
//! You are given an iteger array `cost` where `cost[i]` is the cost of `iᵗʰ` step on a staircase.
//! Once you pay the cost, you can either climb one or two steps.
//!
//! You can either start the step with index `0`, or the step with index `1`.
//!
//! Return *the minimum cost to reach the top of the floor*.
//!
//! ## Constraints:
//!
//! - `2 <= cost.length <= 1000`
//! - `0 <= cost[i] <= 999`

struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();

        if n == 2 {
            return i32::min(cost[0], cost[1]);
        }

        let mut cost_table = vec![0; n];
        cost_table[0] = cost[0];
        cost_table[1] = cost[1];

        for i in 2..n {
            let value = cost[i];
            cost_table[i] = i32::min(cost_table[i - 1] + value, cost_table[i - 2] + value);
        }

        i32::min(cost_table[n - 1], cost_table[n - 2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = vec![10, 15, 20];
        let expected = 15;

        run_test(input, expected);
    }

    #[test]
    fn example_two() {
        let input = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        let expected = 6;

        run_test(input, expected);
    }

    fn run_test(input: Vec<i32>, expected: i32) {
        let result = Solution::min_cost_climbing_stairs(input);

        assert_eq!(result, expected);
    }
}
