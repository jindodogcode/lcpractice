import { ListNode, hasCycle } from "../src/lib";

test("example one", () => {
  const list = [3, 2, 0, -4];
  const pos = 1;
  const head = arrToList(list, pos);
  const output = hasCycle(head);
  const expected = true;

  expect(output).toEqual(expected);
});

test("example two", () => {
  const list = [1, 2];
  const pos = 0;
  const head = arrToList(list, pos);
  const output = hasCycle(head);
  const expected = true;

  expect(output).toEqual(expected);
});

test("example three", () => {
  const list = [1];
  const pos = -1;
  const head = arrToList(list, pos);
  const output = hasCycle(head);
  const expected = false;

  expect(output).toEqual(expected);
});

function arrToList(arr: number[], pos: number): ListNode | null {
  let head: ListNode | null = null;
  let cur: ListNode | null = null;
  let tailNext: ListNode | null = null;

  for (let entry of arr.entries()) {
    const node = new ListNode(entry[1]);

    if (entry[0] === pos) {
      tailNext = node;
    }
    if (entry[0] === arr.length - 1) {
      node.next = tailNext;
    }

    if (head && cur) {
      cur.next = node;
      cur = node;
    } else {
      head = node;
      cur = node;
    }
  }

  return head;
}
