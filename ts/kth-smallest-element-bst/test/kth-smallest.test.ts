import { TreeNode, kthSmallest } from "../src/lib";

test("exmaple one", () => {
  const inputArr = [3, 1, 4, null, 2];
  const k = 1;
  const expected = 1;

  runTest(inputArr, k, expected);
});

test("example two", () => {
  const inputArr = [5, 3, 6, 2, 4, null, null, 1];
  const k = 3;
  const expected = 3;

  runTest(inputArr, k, expected);
});

function runTest(inputArr: (number | null)[], k: number, expected: number) {
  const input = TreeNode.fromArray(inputArr);
  const result = kthSmallest(input, k);

  expect(result).toEqual(expected);
}
