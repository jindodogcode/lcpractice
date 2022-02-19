import assert from "assert";
import { gameOfLife } from "../dist/index.js";

describe("Example One", function () {
  it("", function () {
    const board = [
      [0, 1, 0],
      [0, 0, 1],
      [1, 1, 1],
      [0, 0, 0]
    ];
    gameOfLife(board);
    const expected = [
      [0, 0, 0],
      [1, 0, 1],
      [0, 1, 1],
      [0, 1, 0]
    ];

    assert.deepStrictEqual(board, expected);
  });
});

describe("Example Two", function () {
  it("", function () {
    const board = [
      [1, 1],
      [1, 0]
    ];
    gameOfLife(board);
    const expected = [
      [1, 1],
      [1, 1]
    ];

    assert.deepStrictEqual(board, expected);
  });
});
