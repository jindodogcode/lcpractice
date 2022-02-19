//! # 7. Reverse Integer
//!
//! Given a signed 32-bit integer `x`, return `x` _with its digits reversed_. If reversing `x`
//! causes the value to go outside the signed 32-bit integer range `[-2³¹, 2³¹ - 1]`, then return
//! `0`.
//!
//! **Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
//!
//! ## Constraints:
//! - `-2³¹ <= x <= 2³¹ - 1`

pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = if x >= 0 { 1 } else { -1 };
        let mut num = i32::abs(x);
        let mut rev = 0;

        while num > 0 {
            if rev > i32::max_value() / 10 {
                return 0;
            }

            let digit = num % 10;
            rev = rev * 10 + digit;
            num /= 10;
        }

        rev * sign
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let x = 123;
        let result = Solution::reverse(x);
        let expected = 321;

        assert_eq!(expected, result);
    }

    #[test]
    fn example_two() {
        let x = -123;
        let result = Solution::reverse(x);
        let expected = -321;

        assert_eq!(expected, result);
    }

    #[test]
    fn example_three() {
        let x = 120;
        let result = Solution::reverse(x);
        let expected = 21;

        assert_eq!(expected, result);
    }

    #[test]
    fn example_four() {
        let x = 0;
        let result = Solution::reverse(x);
        let expected = 0;

        assert_eq!(expected, result);
    }
}
