import { MyHashSet } from "../src/lib";

test("example one", () => {
  const myHashSet = new MyHashSet();
  myHashSet.add(1);
  myHashSet.add(2);
  expect(myHashSet.contains(1)).toBeTruthy();
  expect(myHashSet.contains(3)).toBeFalsy();
  myHashSet.add(2);
  expect(myHashSet.contains(2)).toBeTruthy();
  myHashSet.remove(2);
  expect(myHashSet.contains(2)).toBeFalsy();
});
