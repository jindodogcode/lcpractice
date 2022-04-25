import * as Solution from "../src/lib";

test("example one", () => {
  const input = [7, 1, 5, 3, 6, 4];
  const expected = 5;

  runTest(input, expected);
});

test("example two", () => {
  const input = [7, 6, 4, 3, 1];
  const expected = 0;

  runTest(input, expected);
});

function runTest(input: number[], expected: number) {
  const result = Solution.maxProfit(input);

  expect(result).toEqual(expected);
}
