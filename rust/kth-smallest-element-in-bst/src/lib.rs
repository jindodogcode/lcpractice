//! # 230. Kth Smallest Element in a BST
//!
//! Given the `root` of a binary search tree, and an integer `k`, return the `kᵗʰ` *smallest value*
//! (***1-indexed***) *of all the vlaues of the nodes in the tree*.
//!
//! ## Constraints:
//!
//! - The number of nodes in the tree is `n`.
//! - `1 <= k <= n <= 10⁴`
//! - `0 <= Node.val <= 10⁴`

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(vals: &[Option<i32>]) -> Option<Self> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        Some(TreeNode {
            val: vals[0].unwrap(),
            left: Self::from_vec_helper(vals, 1),
            right: Self::from_vec_helper(vals, 2),
        })
    }

    fn from_vec_helper(vals: &[Option<i32>], i: usize) -> Option<Rc<RefCell<Self>>> {
        if i >= vals.len() || vals[i].is_none() {
            return None;
        }

        Some(Rc::new(RefCell::new(TreeNode {
            val: vals[i].unwrap(),
            left: Self::from_vec_helper(vals, i * 2 + 1),
            right: Self::from_vec_helper(vals, i * 2 + 2),
        })))
    }
}

pub struct Solution;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = Vec::new();
        let mut cur = root;
        let mut k = k;

        loop {
            while let Some(node) = cur {
                stack.push(node.clone());
                cur = node.borrow().left.clone();
            }

            cur = stack.pop();
            k -= 1;
            if k == 0 {
                return cur.unwrap().borrow().val;
            }
            let right = cur.as_ref().unwrap().borrow().right.clone();
            cur = right;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input_vec = vec![Some(3), Some(1), Some(4), None, Some(2)];
        let k = 1;
        let expected = 1;

        run_test(input_vec, k, expected);
    }

    #[test]
    fn example_two() {
        let input_vec = vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            None,
            Some(1),
        ];
        let k = 3;
        let expected = 3;

        run_test(input_vec, k, expected);
    }

    fn run_test(input_vec: Vec<Option<i32>>, k: i32, expected: i32) {
        let input = TreeNode::from_vec(&input_vec).map(|r| Rc::new(RefCell::new(r)));
        let result = Solution::kth_smallest(input, k);

        assert_eq!(result, expected);
    }
}
