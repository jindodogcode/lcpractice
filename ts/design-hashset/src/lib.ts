/**
 * # 705. Design HashSet
 *
 * Design a HashSet without using any built-in hash table libraries.
 *
 * Implement `MyHashSet` class:
 * - `void add(key)` Inserts the value `key` into the HashSet.
 * - `bool contains(key)` Returns whether the value `key` exists in the HashSet or not.
 * - `void remove(key)` Removes the value `key` in the HashSet. If `key` does not exist in the HashSet, do nothing.
 *
 * ## Constraints:
 *
 * - `0 <= key <= 10⁶`
 * - At most `10⁴` cals will be made to `add`, `remove`, and `contains`.
 */

class MyHashSet {
  private static size: number = 1009;
  private container: Bucket[];

  constructor() {
    this.container = new Array<Bucket>(MyHashSet.size).fill(new Bucket());
  }

  add(key: number): void {
    this.container[this.hash(key)].add(key);
  }

  remove(key: number): void {
    this.container[this.hash(key)].remove(key);
  }

  contains(key: number): boolean {
    return this.container[this.hash(key)].contains(key);
  }

  private hash(key: number): number {
    return key % MyHashSet.size;
  }
}

class Bucket {
  private container: number[];

  constructor() {
    this.container = [];
  }

  add(key: number): void {
    if (!this.container.includes(key)) {
      this.container.push(key);
    }
  }

  remove(key: number): void {
    const index = this.container.indexOf(key);

    if (index !== -1) {
      this.container.splice(index, 1);
    }
  }

  contains(key: number): boolean {
    return this.container.includes(key);
  }
}

export { MyHashSet };
