import * as Solution from "../src/lib";

test("example one", () => {
  const input = ["https://leetcode.com/problems/design-tinyurl"];
  const expectedIndex = 0;

  runTest(input, expectedIndex);
});

test("multiple inputs", () => {
  const input = [
    "https://leetcode.com/problems/design-tinyurl",
    "https://twitter.com",
    "https://google.com"
  ];
  const expectedIndex = 1;

  runTest(input, expectedIndex);
});

function runTest(input: string[], expectedIndex: number) {
  const tiny = input.map(s => Solution.encode(s));
  const output = Solution.decode(tiny[expectedIndex]);
  const expected = input[expectedIndex];

  console.log(tiny);

  expect(output).toEqual(expected);
}
