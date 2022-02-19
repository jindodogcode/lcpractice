//!
//! # 2. Add Two Numbers
//!
//! You are given two **non-empty** linked lists representing two non-negative integers. The digits
//! are stored in **reverse order**, and each of their nodes contains a single digit. Add the two
//! numbers and return the sum as a linked list.
//!
//! You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//!
//! ## Constraints:
//!
//! - The number of nodes in each linked list is in the range `[1, 100]`.
//! - `0 <= Node.val <= 9`
//! - It is guaranteed that the list represents a number that does not have leading zeros.

struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut out: Option<Box<ListNode>> = None;
        let mut cur: &mut Option<Box<ListNode>> = &mut None;
        let mut carry = 0;

        while l1.is_some() && l2.is_some() {
            let u1 = l1.as_ref().unwrap();
            let u2 = l2.as_ref().unwrap();
            let val = u1.val + u2.val + carry;

            if let Some(node) = cur {
                node.next = Some(Box::new(ListNode::new(val % 10)));
                cur = &mut node.next;
            } else {
                out = Some(Box::new(ListNode::new(val % 10)));
                cur = &mut out;
            }

            carry = val / 10;
            l1 = &u1.next;
            l2 = &u2.next;
        }

        while l1.is_some() {
            let u1 = l1.as_ref().unwrap();
            let val = u1.val + carry;

            if let Some(node) = cur {
                node.next = Some(Box::new(ListNode::new(val % 10)));
                cur = &mut node.next;
            } else {
                out = Some(Box::new(ListNode::new(val % 10)));
                cur = &mut out;
            }

            carry = val / 10;
            l1 = &u1.next;
        }

        while l2.is_some() {
            let u2 = l2.as_ref().unwrap();
            let val = u2.val + carry;

            if let Some(node) = cur {
                node.next = Some(Box::new(ListNode::new(val % 10)));
                cur = &mut node.next;
            } else {
                out = Some(Box::new(ListNode::new(val % 10)));
                cur = &mut out;
            }

            carry = val / 10;
            l2 = &u2.next;
        }

        if carry > 0 {
            cur.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exmaple_one() {
        let l1 = [2, 4, 3];
        let l2 = [5, 6, 4];
        let l1 = arr_to_ll(&l1);
        let l2 = arr_to_ll(&l2);

        let output = Solution::add_two_numbers(l1, l2);

        let expected = [7, 0, 8];
        let expected = arr_to_ll(&expected);

        assert_eq!(output, expected);
    }

    #[test]
    fn example_two() {
        let l1 = [0];
        let l2 = [0];
        let l1 = arr_to_ll(&l1);
        let l2 = arr_to_ll(&l2);

        let output = Solution::add_two_numbers(l1, l2);

        let expected = [0];
        let expected = arr_to_ll(&expected);

        assert_eq!(output, expected);
    }

    #[test]
    fn example_three() {
        let l1 = [9, 9, 9, 9, 9, 9, 9];
        let l2 = [9, 9, 9, 9];
        let l1 = arr_to_ll(&l1);
        let l2 = arr_to_ll(&l2);

        let output = Solution::add_two_numbers(l1, l2);

        let expected = [8, 9, 9, 9, 0, 0, 0, 1];
        let expected = arr_to_ll(&expected);

        assert_eq!(output, expected);
    }

    fn arr_to_ll(arr: &[i32]) -> Option<Box<ListNode>> {
        let mut out: Option<Box<ListNode>> = None;
        let mut cur: &mut Option<Box<ListNode>> = &mut None;
        for val in arr {
            let next = Some(Box::new(ListNode::new(*val)));

            match cur {
                Some(node) => {
                    node.next = next;
                    cur = &mut node.next;
                }
                None => {
                    out = next;
                    cur = &mut out;
                }
            }
        }

        out
    }
}
