import * as Solution from "../src/lib";

test("example one", () => {
  const input = [10, 15, 20];
  const expected = 15;

  runTest(input, expected);
});

test("example two", () => {
  const input = [1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
  const expected = 6;

  runTest(input, expected);
});

function runTest(input: number[], expected: number) {
  const result = Solution.minCostClimbingStairs(input);

  expect(result).toEqual(expected);
}
