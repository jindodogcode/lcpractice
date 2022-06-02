/// # 1143. Longest Common Subsequence
///
/// Given two strings `text1` and `text2`, return *the length of their longest* **common
/// subsequence**, If there is no **common subsequence**, return `0`.
///
/// A **subsequence** of a string is a new string generated from the original string with some
/// characters (can be none) deleted without changing the relative order of the remaining
/// characters.
/// - For example, `"ace"` is a subsequence of `"abcde"`.
///
/// ## Constraints:
/// - `1 <= text1.length, text2.length <= 1000`
/// - `text1` and `text2` consist of only lowercase English characters.

struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let n = text1.len();
        let m = text2.len();

        let mut common_matrix: Vec<Vec<i32>> = vec![vec![0; m]; n];

        for (i, c1) in text1.chars().enumerate() {
            for (j, c2) in text2.chars().enumerate() {
                if c1 == c2 {
                    let prev_value = if i > 0 && j > 0 {
                        common_matrix[i - 1][j - 1]
                    } else {
                        0
                    };
                    common_matrix[i][j] = prev_value + 1;
                } else {
                    let a = if i > 0 { common_matrix[i - 1][j] } else { 0 };
                    let b = if j > 0 { common_matrix[i][j - 1] } else { 0 };
                    common_matrix[i][j] = i32::max(a, b);
                }
            }
        }

        common_matrix[n - 1][m - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input_one = "abcde".to_string();
        let input_two = "ace".to_string();
        let expected = 3;

        run_test(input_one, input_two, expected);
    }

    #[test]
    fn example_two() {
        let input_one = "abc".to_string();
        let input_two = "abc".to_string();
        let expected = 3;

        run_test(input_one, input_two, expected);
    }

    #[test]
    fn example_three() {
        let input_one = "abc".to_string();
        let input_two = "def".to_string();
        let expected = 0;

        run_test(input_one, input_two, expected);
    }

    fn run_test(input_one: String, input_two: String, expected: i32) {
        let result = Solution::longest_common_subsequence(input_one, input_two);

        assert_eq!(expected, result);
    }
}
