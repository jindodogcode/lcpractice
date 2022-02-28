//!
//! # 4. Median of Two Sorted Arrays
//!
//! Given two sorted arrays `nums1` and `nums2` of size `m` and `n` respectively, return **the
//! median** of the two sorted arrays.
//!
//! The overall run time complexity should be `O(log (m+n)).
//!
//! ## Constraints:
//!
//! - `nums1.length == m`
//! - `nums2.length == n`
//! - `0 <= m <= 1000`
//! - `0 <= n <= 1000`
//! - `1 <= m + n <= 2000`
//! - `-10⁶ <= nums1[i], nums2[i] <= 10⁶`

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mid1 = (nums1.len() + nums2.len() + 1) / 2;
    let mid2 = (nums1.len() + nums2.len() + 2) / 2;
    let val = find_at_pos(&nums1, &nums2, mid1);

    if mid1 == mid2 {
        val as f64
    } else {
        (val + find_at_pos(&nums1, &nums2, mid2)) as f64 / 2.0
    }
}

fn find_at_pos(nums1: &[i32], nums2: &[i32], pos: usize) -> i32 {
    let mut start1 = 0;
    let mut start2 = 0;
    let mut pos = pos;

    while pos > 1 {
        let mid1 = start1 + (pos / 2) - 1;
        let mid2 = start2 + (pos / 2) - 1;
        let val1 = if mid1 >= nums1.len() {
            i32::MAX
        } else {
            nums1[mid1]
        };
        let val2 = if mid2 >= nums2.len() {
            i32::MAX
        } else {
            nums2[mid2]
        };

        if val1 < val2 {
            start1 = mid1 + 1;
        } else {
            start2 = mid2 + 1;
        }
        pos -= pos / 2;
    }

    if start1 >= nums1.len() {
        return nums2[start2 + pos - 1];
    }
    if start2 >= nums2.len() {
        return nums1[start1 + pos - 1];
    }

    nums1[start1].min(nums2[start2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let output = find_median_sorted_arrays(nums1, nums2);
        let expected = 2.0;

        assert_eq!(output, expected);
    }

    #[test]
    fn example_two() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        let output = find_median_sorted_arrays(nums1, nums2);
        let expected = 2.5;

        assert_eq!(output, expected);
    }
}
