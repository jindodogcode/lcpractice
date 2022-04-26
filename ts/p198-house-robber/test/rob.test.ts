import * as Solution from "../src/lib";

test("example one", () => {
  const input = [1, 2, 3, 1];
  const expected = 4;

  runTest(input, expected);
});

test("example two", () => {
  const input = [2, 7, 9, 3, 1];
  const expected = 12;

  runTest(input, expected);
});

function runTest(input: number[], expected: number) {
  const result = Solution.rob(input);

  expect(result).toEqual(expected);
}
