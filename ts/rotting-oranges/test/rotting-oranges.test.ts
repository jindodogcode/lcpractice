import { orangesRotting } from "../src/index";

test("Example One", () => {
  const grid = [
    [2, 1, 1],
    [1, 1, 0],
    [0, 1, 1]
  ];
  const result = orangesRotting(grid);
  const expected = 4;

  expect(result).toEqual(expected);
});

test("Example Two", () => {
  const grid = [
    [2, 1, 1],
    [0, 1, 1],
    [1, 0, 1]
  ];
  const result = orangesRotting(grid);
  const expected = -1;

  expect(result).toEqual(expected);
});

test("Example Three", () => {
  const grid = [[0, 2]];
  const result = orangesRotting(grid);
  const expected = 0;

  expect(result).toEqual(expected);
});

test("Failed Test One", () => {
  const grid = [[0, 1]];
  const result = orangesRotting(grid);
  const expected = -1;

  expect(result).toEqual(expected);
});
