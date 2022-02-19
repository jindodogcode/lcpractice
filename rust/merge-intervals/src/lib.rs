//! # 56. Merge Intervals
//!
//! Given an array of _intervals_ where _inverval[i] = [startᵢ, endᵢ]_, merge all overlapping
//! intervals, and return an array of non-overlapping intervals that cover all the intervals the
//! input.
//!
//! ## Constraints:
//! - 1 <= intervals.length <= 10⁴
//! - intervals[i].length = 2
//! - 0 <= startᵢ <= endᵢ <= 10⁴

fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort_by_key(|v| v[0]);
    let mut merged: Vec<Vec<i32>> = Vec::new();

    for interval in intervals.iter() {
        if merged.is_empty() || merged.last().unwrap()[1] < interval[0] {
            merged.push(interval.clone());
        } else {
            let last = merged.last_mut().unwrap();
            if last[1] < interval[1] {
                last[1] = interval[1];
            }
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = Vec::from([
            Vec::from([1, 3]),
            Vec::from([2, 6]),
            Vec::from([8, 10]),
            Vec::from([15, 18]),
        ]);
        let output = merge(input);
        let expected = Vec::from([Vec::from([1, 6]), Vec::from([8, 10]), Vec::from([15, 18])]);

        assert_eq!(output, expected);
    }

    #[test]
    fn example_two() {
        let input = Vec::from([Vec::from([1, 4]), Vec::from([4, 5])]);
        let output = merge(input);
        let expected = Vec::from([Vec::from([1, 5])]);

        assert_eq!(output, expected);
    }
}
