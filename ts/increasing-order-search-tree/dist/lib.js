"use strict";
/**
 * # 897. Increasing Order Search Tree
 * Given the `root` of a binary search tree, rearrange the tree **in-order** so that the leftmost node in the tree is now the root of the tree, and every node has no left child and only one right child.
 *
 * ## Constraints:
 * - The number of nodes in the given tree will be in the range `[1, 100]`.
 * - `0 <= Node.val <= 1000`
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.increasingBST = exports.TreeNode = void 0;
class TreeNode {
    constructor(val, left, right) {
        this.val = val === undefined ? 0 : val;
        this.left = left === undefined ? null : left;
        this.right = right === undefined ? null : right;
    }
    static fromArray(arr) {
        return this.fromArrayHelper(arr, 0);
    }
    static fromArrayHelper(arr, i) {
        if (i >= arr.length || arr[i] === null) {
            return null;
        }
        return new TreeNode(arr[i], this.fromArrayHelper(arr, 2 * i + 1), this.fromArrayHelper(arr, 2 * i + 2));
    }
    static fromArrayRight(arr) {
        return this.fromArrayRightHelper(arr, 0);
    }
    static fromArrayRightHelper(arr, i) {
        if (i >= arr.length || arr[i] === null) {
            return null;
        }
        return new TreeNode(arr[i], null, this.fromArrayRightHelper(arr, i + 1));
    }
}
exports.TreeNode = TreeNode;
function increasingBST(root) {
    const ordered = [];
    inOrder(root, ordered);
    const ans = new TreeNode(0);
    let cur = ans;
    for (let val of ordered) {
        cur.right = new TreeNode(val);
        cur = cur.right;
    }
    return ans.right;
}
exports.increasingBST = increasingBST;
function inOrder(node, vals) {
    if (node === null) {
        return;
    }
    inOrder(node.left, vals);
    vals.push(node.val);
    inOrder(node.right, vals);
}
