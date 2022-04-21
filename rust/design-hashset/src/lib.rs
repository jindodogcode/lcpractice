//! # 705. Design HashSet
//!
//! Design a HashSet without using any  built-in hash table libraries.
//!
//! Implement `MyHashSet` class:
//! - `void add(key)` Inserts the value `key` into the HashSet.
//! - `bool contains(key)` Returns whether the value `key` exists in the HashSet or not.
//! - `void remove(key)` Removes the value `key` in the HashSet. If `key` does not exist in the
//! HashSet, do nothing.
//!
//! ## Constraints:
//!
//! - `0 <= key <= 10⁶`
//! - At most `10⁴` calss will be made to `add`, `remove`, and `contains`.

const SIZE: usize = 1009;

#[derive(Debug, Clone, Default)]
pub struct MyHashSet {
    container: Vec<Bucket>,
}

impl MyHashSet {
    pub fn new() -> Self {
        MyHashSet {
            container: vec![Bucket::new(); SIZE],
        }
    }

    pub fn add(&mut self, key: i32) {
        self.container[MyHashSet::hash(key)].add(key);
    }

    pub fn remove(&mut self, key: i32) {
        self.container[MyHashSet::hash(key)].remove(key);
    }

    pub fn contains(&self, key: i32) -> bool {
        self.container[MyHashSet::hash(key)].contains(key)
    }

    fn hash(key: i32) -> usize {
        (key as usize) % SIZE
    }
}

#[derive(Debug, Clone, Default)]
struct Bucket {
    container: Vec<i32>,
}

impl Bucket {
    fn new() -> Self {
        Bucket {
            container: Vec::new(),
        }
    }

    fn add(&mut self, key: i32) {
        let index = self.container.iter().position(|&v| v == key);

        if index.is_none() {
            self.container.push(key);
        }
    }

    fn remove(&mut self, key: i32) {
        let index = self.container.iter().position(|&v| v == key);

        if let Some(i) = index {
            self.container.remove(i);
        }
    }

    fn contains(&self, key: i32) -> bool {
        self.container.contains(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::MyHashSet;

    #[test]
    fn example_one() {
        let mut mhs = MyHashSet::new();
        mhs.add(1);
        mhs.add(2);
        assert!(mhs.contains(1));
        assert!(!mhs.contains(3));
        mhs.add(2);
        assert!(mhs.contains(2));
        mhs.remove(2);
        assert!(!mhs.contains(2));
    }
}
