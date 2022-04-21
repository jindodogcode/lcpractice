import { MyHashMap } from "../src/lib";

test("example one", () => {
  const mhm = new MyHashMap();
  mhm.put(1, 1);
  mhm.put(2, 2);
  expect(mhm.get(1)).toEqual(1);
  expect(mhm.get(3)).toEqual(-1);
  mhm.put(2, 1);
  expect(mhm.get(2)).toEqual(1);
  mhm.remove(2);
  expect(mhm.get(2)).toEqual(-1);
});
