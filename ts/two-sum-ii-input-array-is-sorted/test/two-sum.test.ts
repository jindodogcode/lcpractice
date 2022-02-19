import { twoSum } from "../src/index";

test("Example One", () => {
  const numbers = [2, 7, 11, 15];
  const target = 9;
  const result = twoSum(numbers, target);
  const expected = [1, 2];

  expect(expected).toEqual(result);
});

test("Example Two", () => {
  const numbers = [2, 3, 4];
  const target = 6;
  const result = twoSum(numbers, target);
  const expected = [1, 3];

  expect(expected).toEqual(result);
});

test("Example Three", () => {
  const numbers = [-1, 0];
  const target = -1;
  const result = twoSum(numbers, target);
  const expected = [1, 2];

  expect(expected).toEqual(result);
});
