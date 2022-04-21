"use strict";
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
    constructor() {
        this.container = new Array(MyHashSet.size);
    }
    add(key) {
        this.container[this.hash(key)] = true;
    }
    remove(key) {
        this.container[this.hash(key)] = false;
    }
    contains(key) {
        return this.container[this.hash(key)];
    }
    hash(key) {
        return key % MyHashSet.size;
    }
}
MyHashSet.size = 1009;
