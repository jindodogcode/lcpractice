//! # 1249. Minimum Remove to make valid Parentheses
//!
//! Given a string s of `'('`, `')' and lowercase English charecters.
//!
//! Your task is to remove the minimum number of parentheses (`'('` or `')'`, in any positions) so
//! that the resulting *parentheses string* is valid and return **any** valid string.
//!
//! Formally, a *parentheses string* is valid if and only if:
//! - It is the empty string, contains only lowercase characters, or
//! - It can be written as `AB` (`A` concatenated with `B`), where `A` and `B` are valid strings,
//! or
//! - It can be written as `(A)`, where `A` is a valid string.
//!
//! ## Constraints:
//!
//! - `1 <- s.length <= 10⁵`
//! - `s[i]` is either `'('`, `')'`, or lowercase English letter.

struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut count = 0;
        let mut output = String::with_capacity(s.len());

        for c in s.chars() {
            match c {
                '(' => {
                    count += 1;
                    output.push(c);
                }
                ')' => {
                    if count > 0 {
                        count -= 1;
                        output.push(c);
                    }
                }
                _ => output.push(c),
            }
        }

        if count > 0 {
            let mut second = String::with_capacity(output.len());
            for c in output.chars().rev() {
                match c {
                    '(' => {
                        if count > 0 {
                            count -= 1;
                        } else {
                            second.push(c);
                        }
                    }
                    _ => second.push(c),
                }
            }

            output = second.chars().rev().collect();
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = "lee(t(c)o)de)";
        let expected = ["lee(t(c)o)de", "lee(t(co)de)", "lee(t(c)ode)"];

        run_test(input, &expected);
    }

    #[test]
    fn example_two() {
        let input = "a)b(c)d";
        let expected = ["ab(c)d"];

        run_test(input, &expected);
    }

    #[test]
    fn example_three() {
        let input = "))((";
        let expected = [""];

        run_test(input, &expected);
    }

    #[test]
    fn failed_one() {
        let input = "())()(((";
        let expected = ["()()"];

        run_test(input, &expected);
    }

    fn run_test(input: &str, expected: &[&str]) {
        let result = Solution::min_remove_to_make_valid(input.to_owned());
        assert!(expected.iter().any(|e| *e == result));
    }
}
