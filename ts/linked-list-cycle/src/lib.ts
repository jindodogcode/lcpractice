/**
 * # 141. Linked List Cycle
 *
 * Given `head`, the head of a linked list, determine if the linked list has a cycle in it.
 *
 * There is a cycle in a linked list if there is some node in the list that can be reached again by continously following the `next` pointer. Internally, `pos` is used to denote the index of the node that tail's `next` pointer is conntected to. **Note that `pos` is not passed as a parameter**.
 *
 * Return `true` *if there is a cycle in the linked list*. Otherwise, return `false`.
 *
 * ## Constraints:
 *
 * - The number of nodes in the list is in the range `[0, 10⁴]`.
 * - `-10⁵ <= Node.val <= 10⁵`
 * - `pos` is `-1` or a **valid index** in the linked-list.
 *
 * **Follow up:** Can you solve it using `O(1)` (i.e. constant) memory?
 */

class ListNode {
  val: number;
  next: ListNode | null;

  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val;
    this.next = next == undefined ? null : next;
  }
}

function hasCycle(head: ListNode | null): boolean {
  if (!head) {
    return false;
  }

  let node = head;
  const seen = new Set();

  while (node.next) {
    if (seen.has(node)) {
      return true;
    } else {
      seen.add(node);
    }

    node = node.next;
  }

  return false;
}

export { ListNode, hasCycle };
