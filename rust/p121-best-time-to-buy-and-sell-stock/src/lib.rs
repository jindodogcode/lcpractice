//! # 121. Best Time To Buy and Sell Stock
//!
//! You are given an array `prices` where `prices[i]` is the price of a given stock on the `iᵗʰ`
//! day.
//!
//! You want to maximize your profit by choosing a **single day** to buy one stock and choosing a
//! **different day in the future** to sell that stock.
//!
//! Return *the maximum profit you can achieve from this transaction*. If you cannoty achieve any
//! profit, return `0`.
//!
//! ## Constraints:
//!
//! - `1 <= prices.length <= 10⁵`
//! - `0 <= prices[i] <= 10⁴`

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_profit = 0;

        for price in prices {
            max_profit = max_profit.max(price - min_price);
            min_price = min_price.min(price);
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = vec![7, 1, 5, 3, 6, 4];
        let expected = 5;

        run_test(input, expected);
    }

    #[test]
    fn example_two() {
        let input = vec![7, 6, 4, 3, 1];
        let expected = 0;

        run_test(input, expected);
    }

    fn run_test(input: Vec<i32>, expected: i32) {
        let result = Solution::max_profit(input);

        assert_eq!(result, expected);
    }
}
