import * as Solution from "../src/lib";

test("example one", () => {
  const input = 4;
  const expected = 4;

  runTest(input, expected);
});

test("example two", () => {
  const input = 25;
  const expected = 1389537;

  runTest(input, expected);
});

function runTest(input: number, expected: number) {
  const result = Solution.tribonacci(input);

  expect(result).toEqual(expected);
}
