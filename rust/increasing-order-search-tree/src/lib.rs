//! # 897. Increasing Order Search Tree
//!
//! Given the `root` of a binary search tree, rearrange the tree in **in-order** so that the
//! leftmost node in the tree is now the root of the tree, and every node has no left child and
//! only one right child.
//!
//! ## Constraints:
//!
//! - The number of nodes in the given tree will be in the range `[1, 100]`.
//! - `0 <= Node.val <= 1000`

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

    #[inline]
    pub fn from_vec(vals: &Vec<Option<i32>>) -> Option<Self> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        Some(TreeNode {
            val: vals[0].unwrap(),
            left: Self::from_vec_helper(vals, 1),
            right: Self::from_vec_helper(vals, 2),
        })
    }

    fn from_vec_helper(vals: &Vec<Option<i32>>, i: usize) -> Option<Rc<RefCell<Self>>> {
        if i >= vals.len() || vals[i].is_none() {
            return None;
        }

        Some(Rc::new(RefCell::new(TreeNode {
            val: vals[i].unwrap(),
            left: Self::from_vec_helper(vals, i * 2 + 1),
            right: Self::from_vec_helper(vals, i * 2 + 2),
        })))
    }

    #[inline]
    pub fn from_vec_right(vals: &Vec<Option<i32>>) -> Option<Self> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        Some(TreeNode {
            val: vals[0].unwrap(),
            left: None,
            right: Self::from_vec_right_helper(vals, 1),
        })
    }

    fn from_vec_right_helper(vals: &Vec<Option<i32>>, i: usize) -> Option<Rc<RefCell<Self>>> {
        if i >= vals.len() || vals[i].is_none() {
            return None;
        }

        Some(Rc::new(RefCell::new(TreeNode {
            val: vals[i].unwrap(),
            left: None,
            right: Self::from_vec_right_helper(vals, i + 1),
        })))
    }
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val && self.left.eq(&other.left) && self.right.eq(&other.right)
    }
}

struct Solution;

impl Solution {
    fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut order = Vec::new();
        Self::find_order(root, &mut order);

        let ans = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut cur = ans.clone();

        for val in order {
            let right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            cur.borrow_mut().right = right.clone();
            cur = right.clone().unwrap();
        }

        let ans = ans.borrow().right.clone();
        ans
    }

    fn find_order(maybe_node: Option<Rc<RefCell<TreeNode>>>, order: &mut Vec<i32>) {
        if let Some(node) = maybe_node {
            Self::find_order(node.borrow().left.clone(), order);
            order.push(node.borrow().val);
            Self::find_order(node.borrow().right.clone(), order);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input_vec = vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            Some(8),
            Some(1),
            None,
            None,
            None,
            None,
            None,
            Some(7),
            Some(9),
        ];
        let input = TreeNode::from_vec(&input_vec).map(|r| Rc::new(RefCell::new(r)));
        let result = Solution::increasing_bst(input);
        let expected_vec = vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
        ];
        let expected = TreeNode::from_vec_right(&expected_vec).map(|r| Rc::new(RefCell::new(r)));

        assert_eq!(result, expected);
    }

    #[test]
    fn example_two() {
        let input_vec = vec![Some(5), Some(1), Some(7)];
        let input = TreeNode::from_vec(&input_vec).map(|r| Rc::new(RefCell::new(r)));
        let result = Solution::increasing_bst(input);
        let expected_vec = vec![Some(1), Some(5), Some(7)];
        let expected = TreeNode::from_vec_right(&expected_vec).map(|r| Rc::new(RefCell::new(r)));

        assert_eq!(result, expected);
    }
}
