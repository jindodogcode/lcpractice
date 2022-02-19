import assert from "assert";
import { twoSum } from "../dist/index.js";

describe("Example One", function () {
  it("Should be [0, 1]", function () {
    const nums = [2, 7, 11, 15];
    const target = 9;
    const output = twoSum(nums, target);
    const expected = [0, 1];

    assert.deepStrictEqual(output, expected);
  });
});

describe("Example Two", function () {
  it("Should be [1, 2]", function () {
    const nums = [3, 2, 4];
    const target = 6;
    const output = twoSum(nums, target);
    const expected = [1, 2];

    assert.deepStrictEqual(output, expected);
  });
});

describe("Example Three", function () {
  it("Should be [0, 1]", function () {
    const nums = [3, 3];
    const target = 6;
    const output = twoSum(nums, target);
    const expected = [0, 1];

    assert.deepStrictEqual(output, expected);
  });
});
