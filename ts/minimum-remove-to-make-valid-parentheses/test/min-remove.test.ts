import { minRemoveToMakeValid } from "../src/lib";

test("example one", () => {
  const input = "lee(t(c)o)de)";
  const expected = ["lee(t(c)o)de", "lee(t(co)de)", "lee(t(c)ode)"];

  runTest(input, expected);
});

test("example two", () => {
  const input = "a)b(c)d";
  const expected = ["ab(c)d"];

  runTest(input, expected);
});

test("example three", () => {
  const input = "))((";
  const expected = [""];

  runTest(input, expected);
});

test("failed one", () => {
  const input = "())()(((";
  const expected = ["()()"];

  runTest(input, expected);
});

function runTest(input: string, expected: string[]) {
  const result = minRemoveToMakeValid(input);
  expect(expected).toContain(result);
}
