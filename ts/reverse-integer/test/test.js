import { assert } from "chai";
import { reverse } from "../dist/index.js";

describe("Example One", function () {
  it("Expects 321", function () {
    const x = 123;
    const result = reverse(x);
    const expected = 321;

    assert.strictEqual(result, expected);
  });
});

describe("Example Two", function () {
  it("Expects -321", function () {
    const x = -123;
    const result = reverse(x);
    const expected = -321;

    assert.strictEqual(result, expected);
  });
});

describe("Example Three", function () {
  it("Expects 21", function () {
    const x = 120;
    const result = reverse(x);
    const expected = 21;

    assert.strictEqual(result, expected);
  });
});

describe("Example Four", function () {
  it("Expects 0", function () {
    const x = 0;
    const result = reverse(x);
    const expected = 0;

    assert.strictEqual(result, expected);
  });
});
