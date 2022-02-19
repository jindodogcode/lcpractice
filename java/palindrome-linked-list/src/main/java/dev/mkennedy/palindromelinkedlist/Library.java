/*
 * Given a singly linked list, determine if it is a palindrome.
 *
 * Follow up:
 * Could you do it in O(n) time and O(1) space?
 */
package dev.mkennedy.palindromelinkedlist;

import java.util.ArrayList;
import java.util.List;

public class Library {
    public static boolean isPalindrome(ListNode head) {
        List<Integer> list = new ArrayList<>();
        ListNode node = head;

        while (node != null) {
            list.add(node.val);
            node = node.next;
        }

        int i = 0;
        int j = list.size() - 1;
        while (i < j) {
            if (!list.get(i).equals(list.get(j))) return false;
            i += 1;
            j -= 1;
        }

        return true;
    }

    public static ListNode makeList(int[] nums) {
        ListNode preHead = new ListNode();
        ListNode cur = preHead;

        for (int val : nums) {
            cur.next = new ListNode(val);
            cur = cur.next;
        }

        return preHead.next;
    }

    public static class ListNode {
        int val;
        ListNode next;

        ListNode() {}
        ListNode(int val) { this.val = val; }
        ListNode(int val, ListNode next) { this.val = val; this.next = next; }
    }
}
