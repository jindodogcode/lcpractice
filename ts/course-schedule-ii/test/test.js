import { assert } from "chai";
import { findOrder } from "../dist/index.js";

describe("Example One", function () {
  it("expecting [0, 1]", function () {
    const numCourses = 2;
    const prerequisites = [[1, 0]];
    const output = findOrder(numCourses, prerequisites);
    const expected = [0, 1];

    assert.deepEqual(output, expected);
  });
});

describe("Example Two", function () {
  it("expecting [0, 2, 1, 3]", function () {
    const numCourses = 4;
    const prerequisites = [
      [1, 0],
      [2, 0],
      [3, 1],
      [3, 2]
    ];
    const output = findOrder(numCourses, prerequisites);
    const expected = [
      [0, 1, 2, 3],
      [0, 2, 1, 3]
    ];

    assert.deepInclude(expected, output);
  });
});

describe("Example Three", function () {
  it("expecting [0]", function () {
    const numCourses = 1;
    const prerequisites = [];
    const output = findOrder(numCourses, prerequisites);
    const expected = [0];

    assert.deepEqual(output, expected);
  });
});
