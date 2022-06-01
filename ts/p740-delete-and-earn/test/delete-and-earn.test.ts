import * as Solution from "../src/lib";

test("example one", () => {
  const input = [3, 4, 2];
  const expected = 6;

  runTest(input, expected);
});

test("example two", () => {
  const input = [2, 2, 3, 3, 3, 4];
  const expected = 9;

  runTest(input, expected);
});

function runTest(input: number[], expected: number) {
  const result = Solution.deleteAndEarn(input);

  expect(result).toEqual(expected);
}
