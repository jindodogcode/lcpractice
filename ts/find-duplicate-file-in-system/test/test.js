import { assert } from "chai";
import { findDuplicate } from "../dist/index.js";

describe("Example One", function () {
  it("", function () {
    const paths = [
      "root/a 1.txt(abcd) 2.txt(efgh)",
      "root/c 3.txt(abcd)",
      "root/c/d 4.txt(efgh)",
      "root 4.txt(efgh)"
    ];
    const output = findDuplicate(paths);
    const expected = [
      ["root/a/2.txt", "root/c/d/4.txt", "root/4.txt"],
      ["root/a/1.txt", "root/c/3.txt"]
    ];

    assert.equal(output.length, expected.length);
    for (let paths of output) {
      assert.deepInclude(expected, paths);
    }
  });
});

describe("Example Two", function () {
  it("", function () {
    const paths = [
      "root/a 1.txt(abcd) 2.txt(efgh)",
      "root/c 3.txt(abcd)",
      "root/c/d 4.txt(efgh)"
    ];
    const output = findDuplicate(paths);
    const expected = [
      ["root/a/2.txt", "root/c/d/4.txt"],
      ["root/a/1.txt", "root/c/3.txt"]
    ];

    assert.equal(output.length, expected.length);
    for (let paths of output) {
      assert.deepInclude(expected, paths);
    }
  });
});
