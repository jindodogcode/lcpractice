import * as lib from "../src/lib";

test("example one", () => {
  const numCourses = 2;
  const prerequisites = [[1, 0]];
  const output = lib.v1.findOrder(numCourses, prerequisites);
  const expected = [0, 1];

  expect(output).toEqual(expected);
});

test("example two", () => {
  const numCourses = 4;
  const prerequisites = [
    [1, 0],
    [2, 0],
    [3, 1],
    [3, 2]
  ];
  const output = lib.v1.findOrder(numCourses, prerequisites);
  const expected = [
    [0, 2, 1, 3],
    [0, 1, 2, 3]
  ];

  expect(expected).toContainEqual(output);
});

test("example three", () => {
  const numCourses = 1;
  const prerequisites: number[][] = [];
  const output = lib.v1.findOrder(numCourses, prerequisites);
  const expected = [0];

  expect(output).toEqual(expected);
});
