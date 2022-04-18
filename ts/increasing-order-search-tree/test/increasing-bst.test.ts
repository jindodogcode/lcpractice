import { TreeNode, increasingBST } from "../src/lib";

test("example one", () => {
  const inputArr = [
    5,
    3,
    6,
    2,
    4,
    null,
    8,
    1,
    null,
    null,
    null,
    null,
    null,
    7,
    9
  ];
  const input = TreeNode.fromArray(inputArr);
  const output = increasingBST(input);
  const expectedArr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
  const expected = TreeNode.fromArrayRight(expectedArr);

  console.log(output);
  console.log(expected);

  expect(output).toEqual(expected);
});

test("example two)", () => {
  const inputArr = [5, 1, 7];
  const input = TreeNode.fromArray(inputArr);
  const output = increasingBST(input);
  const expectedArr = [1, 5, 7];
  const expected = TreeNode.fromArrayRight(expectedArr);

  expect(output).toEqual(expected);
});
