//! # 1137. N-th Tribonacci Number
//!
//! The Tribonacci sequence Tₙ is defined as follows:
//! T₀=0, T₁=1, T₂=1, and Tₙ₊₃=Tₙ+Tₙ₊₁+Tₙ₊₂ for n > 0.
//! Given `n`, return the value of `Tₙ`.
//!
//! ## Constraints:
//!
//! - `0 <= n <= 37`
//! - The answer is guaranteed to fit within a 32-bit integer, ie. `answer <= 2³¹ - 1`.

struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0 {
            return 0;
        } else if n == 1 || n == 2 {
            return 1;
        }

        let mut t0 = 0;
        let mut t1 = 1;
        let mut t2 = 1;
        let mut ans = 0;

        for _ in 3..=n {
            ans = t0 + t1 + t2;
            t0 = t1;
            t1 = t2;
            t2 = ans;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = 4;
        let expected = 4;

        run_test(input, expected);
    }

    #[test]
    fn example_two() {
        let input = 25;
        let expected = 1389537;

        run_test(input, expected);
    }

    fn run_test(input: i32, expected: i32) {
        let result = Solution::tribonacci(input);

        assert_eq!(expected, result);
    }
}
