//! # 706. Design HashMap
//!
//! Design a HashMap without using any built-in hash table libraries.
//!
//! Implement the `MyHashMap` class:
//! - `MyHashMap()` initializes the object with an empty map.
//! - `void put(int key, int value)` inserts a `(key, value)` pair into the HashMap. If the `key`
//! already exists in the map, update the corresponding `value`.
//! - `int get(int key)` returns the `value` to which the specified `key` is mapped, or `-1` if this
//! map contains no mapping for the `key`.
//! - `void remove(key)` remove the `key` and its corresponding `value` if the map contains the
//! mapping for the `key`.
//!
//! ## Constraints:
//!
//! - `0 <= key, value <= 10⁶`
//! - At most `10⁴` calss will be made to `put`, `get`, and `remove`.

#[derive(Default, Debug)]
pub struct MyHashMap {
    container: Vec<Bucket>,
}

impl MyHashMap {
    const SIZE: usize = 1009;

    pub fn new() -> Self {
        MyHashMap {
            container: vec![Bucket::new(); Self::SIZE],
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.container[Self::hash(key)].put(key, value);
    }

    pub fn get(&self, key: i32) -> i32 {
        self.container[Self::hash(key)].get(key)
    }

    pub fn remove(&mut self, key: i32) {
        self.container[Self::hash(key)].remove(key);
    }

    fn hash(key: i32) -> usize {
        (key as usize) % Self::SIZE
    }
}

#[derive(Debug, Default, Clone)]
struct Bucket {
    container: Vec<Pair>,
}

impl Bucket {
    fn new() -> Self {
        Bucket {
            container: Vec::new(),
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let index = self.container.iter().position(|p| p.key == key);

        if let Some(i) = index {
            self.container[i].value = value;
        } else {
            self.container.push(Pair { key, value });
        }
    }

    fn get(&self, key: i32) -> i32 {
        self.container
            .iter()
            .find(|p| p.key == key)
            .map(|p| p.value)
            .unwrap_or(-1)
    }

    fn remove(&mut self, key: i32) {
        let index = self.container.iter().position(|p| p.key == key);

        if let Some(i) = index {
            self.container.remove(i);
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Pair {
    key: i32,
    value: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut mhm = MyHashMap::new();
        mhm.put(1, 1);
        mhm.put(2, 2);
        assert_eq!(mhm.get(1), 1);
        assert_eq!(mhm.get(3), -1);
        mhm.put(2, 1);
        assert_eq!(mhm.get(2), 1);
        mhm.remove(2);
        assert_eq!(mhm.get(2), -1);
    }
}
