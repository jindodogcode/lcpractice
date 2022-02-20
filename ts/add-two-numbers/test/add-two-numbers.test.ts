import { addTwoNumbers, ListNode } from "../src/index";

test("example one", () => {
  const a1 = [2, 4, 3];
  const a2 = [5, 6, 4];
  const l1 = arrayToLinkedList(a1);
  const l2 = arrayToLinkedList(a2);

  const output = addTwoNumbers(l1, l2);

  const expectedArr = [7, 0, 8];
  const expected = arrayToLinkedList(expectedArr);

  expect(output).toEqual(expected);
});

test("example two", () => {
  const a1 = [0];
  const a2 = [0];
  const l1 = arrayToLinkedList(a1);
  const l2 = arrayToLinkedList(a2);

  const output = addTwoNumbers(l1, l2);

  const expectedArr = [0];
  const expected = arrayToLinkedList(expectedArr);

  expect(output).toEqual(expected);
});

test("example three", () => {
  const a1 = [9, 9, 9, 9, 9, 9, 9];
  const a2 = [9, 9, 9, 9];
  const l1 = arrayToLinkedList(a1);
  const l2 = arrayToLinkedList(a2);

  const output = addTwoNumbers(l1, l2);

  const expectedArr = [8, 9, 9, 9, 0, 0, 0, 1];
  const expected = arrayToLinkedList(expectedArr);

  expect(output).toEqual(expected);
});

function arrayToLinkedList(arr: number[]): ListNode | null {
  let out: ListNode | null = null;
  let cur: ListNode | null = null;

  for (let n of arr) {
    if (cur) {
      cur.next = new ListNode(n);
      cur = cur.next;
    } else {
      out = new ListNode(n);
      cur = out;
    }
  }

  return out;
}
