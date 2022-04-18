/**
 * # 897. Increasing Order Search Tree
 * Given the `root` of a binary search tree, rearrange the tree **in-order** so that the leftmost node in the tree is now the root of the tree, and every node has no left child and only one right child.
 *
 * ## Constraints:
 * - The number of nodes in the given tree will be in the range `[1, 100]`.
 * - `0 <= Node.val <= 1000`
 */

class TreeNode {
  val: number;
  left: TreeNode | null;
  right: TreeNode | null;

  constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = val === undefined ? 0 : val;
    this.left = left === undefined ? null : left;
    this.right = right === undefined ? null : right;
  }

  static fromArray(arr: (number | null)[]): TreeNode | null {
    return this.fromArrayHelper(arr, 0);
  }

  private static fromArrayHelper(
    arr: (number | null)[],
    i: number
  ): TreeNode | null {
    let val = arr[i];
    if (i >= arr.length || !val) {
      return null;
    }

    return new TreeNode(
      val,
      this.fromArrayHelper(arr, 2 * i + 1),
      this.fromArrayHelper(arr, 2 * i + 2)
    );
  }

  static fromArrayRight(arr: number[]): TreeNode | null {
    return this.fromArrayRightHelper(arr, 0);
  }

  private static fromArrayRightHelper(
    arr: number[],
    i: number
  ): TreeNode | null {
    const val = arr[i];
    if (i >= arr.length || !val) {
      return null;
    }

    return new TreeNode(val, null, this.fromArrayRightHelper(arr, i + 1));
  }
}

function increasingBST(root: TreeNode | null): TreeNode | null {
  const ordered: number[] = [];
  inOrder(root, ordered);
  const ans = new TreeNode(0);
  let cur = ans;

  for (let val of ordered) {
    cur.right = new TreeNode(val);
    cur = cur.right;
  }

  return ans.right;
}

function inOrder(node: TreeNode | null, vals: number[]) {
  if (node === null) {
    return;
  }

  inOrder(node.left, vals);
  vals.push(node.val);
  inOrder(node.right, vals);
}

export { TreeNode, increasingBST };
