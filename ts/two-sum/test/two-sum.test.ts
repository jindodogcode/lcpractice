import { twoSum } from "../src/lib";

test("example one", () => {
  const nums = [2, 7, 11, 15];
  const target = 9;
  const output = twoSum(nums, target);
  const expected = [0, 1];

  expect(output).toEqual(expected);
});

test("example two", () => {
  const nums = [3, 2, 4];
  const target = 6;
  const output = twoSum(nums, target);
  const expected = [1, 2];

  expect(output).toEqual(expected);
});

test("exmaple three", () => {
  const nums = [3, 3];
  const target = 6;
  const output = twoSum(nums, target);
  const expected = [0, 1];

  expect(output).toEqual(expected);
});
