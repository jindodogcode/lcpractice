/**
 * # 230. Kth Smallest Element in a BST
 *
 * Given the `root` of a binary search tree, and an integer `k`, return the `kᵗʰ` *smallest value* (***1-indexed***) *of all the values of the nodes in the tree*.
 *
 * ## Constraints:
 *
 * - The number of nodes in the tree is `n`.
 * - `1 <= k <= n <= 10⁴`
 * - `0 <- Node.val <= 10⁴`
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
    const val = arr[i];
    if (!val) {
      return null;
    }

    return new TreeNode(
      val,
      this.fromArrayHelper(arr, i * 2 + 1),
      this.fromArrayHelper(arr, i * 2 + 2)
    );
  }
}

function kthSmallest(root: TreeNode | null, k: number): number {
  const stack = new Array<TreeNode>();
  let cur: TreeNode | null | undefined = root;
  let count = k;

  while (true) {
    while (cur) {
      stack.push(cur);
      cur = cur.left;
    }

    cur = stack.pop();
    count -= 1;
    if (!cur) {
      throw Error();
    }

    if (count === 0) {
      return cur.val;
    }
    cur = cur.right;
  }
}

export { TreeNode, kthSmallest };
