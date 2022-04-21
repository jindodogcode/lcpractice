/**
 * # 706. Design HashMap
 *
 * Design a HashMap without using any built-in hash table libraries.
 *
 * Implement the `MyHashMap` class:
 * - `MyHashMap()` initializes the object with an empty map.
 * - `void put(int key, int value)` inserts a `(key, value)` pair into the HashMap. If the `key` already exists in the map. update the corresponding `value`.
 * - `int get(int key)` returns the `value` to which the specified `key` is mapped, or `-1` if this map contains no mapping for the `key`.
 * - `void remove(key)` removes the `key` and its corresponding `value` if the map contains the mapping for the `key`.
 *
 * ## Constraints:
 * - `0 <= key, value <= 10⁶`
 * - At most `10⁴` calls will be made to `put`, `get`, and `remove`.
 */

class MyHashMap {
  private static SIZE = 1009;
  container: Array<Bucket>;

  constructor() {
    this.container = Array<Bucket>(MyHashMap.SIZE).fill(new Bucket());
  }

  put(key: number, value: number): void {
    let hash = MyHashMap.hash(key);
    this.container[hash].put(key, value);
  }

  get(key: number): number {
    let hash = MyHashMap.hash(key);
    return this.container[hash].get(key);
  }

  remove(key: number): void {
    let hash = MyHashMap.hash(key);
    this.container[hash].remove(key);
  }

  private static hash(key: number): number {
    return key % MyHashMap.SIZE;
  }
}

class Bucket {
  container: Array<Pair>;

  constructor() {
    this.container = new Array<Pair>();
  }

  put(key: number, value: number): void {
    const index = this.container.findIndex(value => value.key === key);

    if (index === -1) {
      this.container.push(new Pair(key, value));
    } else {
      this.container[index].value = value;
    }
  }

  get(key: number): number {
    const index = this.container.findIndex(value => value.key === key);

    if (index !== -1) {
      return this.container[index].value;
    } else {
      return -1;
    }
  }

  remove(key: number): void {
    const index = this.container.findIndex(value => value.key === key);

    if (index !== -1) {
      this.container.splice(index, 1);
    }
  }
}

class Pair {
  key: number;
  value: number;

  constructor(key: number, value: number) {
    this.key = key;
    this.value = value;
  }
}

export { MyHashMap };
