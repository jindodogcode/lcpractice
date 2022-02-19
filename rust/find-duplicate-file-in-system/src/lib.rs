//! # 609. Find Duplicate File in System
//!
//! Given a list `paths` of directory info, including the directory path, and all the files with
//! contents in this directory, return _all the duplicate files in the files system in terms of
//! theirs paths_. You may return the answer in __any order__.
//!
//! A group of duplicate files consists of at least two files that have the same content.
//!
//! A single directory info string in the input list hast the following format:
//! - `"root/d1/d2/../dm f1.txt(f1_content) f2.txt(f2_content) ... fn.txt(fn_content)"`
//!
//! It means there are `n` files (f1.txt, f2.txt ... fn.txt) with content `(f1_content, f2_content
//! ... fn_content)` respectively in the directory `"root/d1/d2/../dm"`. Note that `n >= 0` and `m
//! >= 0`. If `m = 0`, it means the directory is just the root directory.
//!
//! The output is a list of groups of duplicate files paths. For each group, it contains all the
//! file paths of the files that have the same content. A file path is a string that has the
//! following format:
//! - `"directory_path/file_name.txt"`
//!
//! ## Constraints:
//! - `1 <= paths.length <- 2 * 10⁴`
//! - `1 <= paths[i].length <- 3000`
//! - `1 <= sum(paths[i].length) <= 5 * 10⁵`
//! - `paths[i]` consist of English letters, digits, '/', '.', '(', ')', and ' '.
//! - You may assume no files or directories share the same name in the same directory.
//! - You may assume each given directory info represents a unique directory. A single blank space
//! separates the directory path and file info.
//!
//! ## Follow up:
//! - Imagine you are given a real file system, how will you search files? DFS or BFS?
//! - If the file content is very large (GB level), how will you modify your solution?
//! - If you can only read the file by 1kb each time, how will you modify your solution?
//! - What is the time complexity of your modified solution? What is the most time-consuming part
//! and memory-consuming part of it? How to optimize?
//! - How to make sure the duplicated files you find are not false positive?

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_duplicates(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut content_map: HashMap<String, Vec<String>> = HashMap::new();

        for full in paths.into_iter() {
            let mut iter = full.split_whitespace();
            let path = iter.next().unwrap();

            for file in iter {
                let (f_name, f_content) = file.split_once('(').unwrap();
                let f_content = f_content.trim_end_matches(')');
                content_map
                    .entry(f_content.to_string())
                    .or_default()
                    .push(format!("{}/{}", path, f_name));
            }
        }

        content_map.into_values().filter(|v| v.len() > 1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input: Vec<String> = vec![
            "root/a 1.txt(abcd) 2.txt(efgh)".into(),
            "root/c 3.txt(abcd)".into(),
            "root/c/d 4.txt(efgh)".into(),
            "root 4.txt(efgh)".into(),
        ];
        let output = Solution::find_duplicates(input);
        let expected: Vec<Vec<String>> = vec![
            vec![
                "root/a/2.txt".into(),
                "root/c/d/4.txt".into(),
                "root/4.txt".into(),
            ],
            vec!["root/a/1.txt".into(), "root/c/3.txt".into()],
        ];

        assert_eq!(expected, output);
    }

    #[test]
    fn example_two() {
        let input: Vec<String> = vec![
            "root/a 1.txt(abcd) 2.txt(efgh)".into(),
            "root/c 3.txt(abcd)".into(),
            "root/c/d 4.txt(efgh)".into(),
        ];
        let output = Solution::find_duplicates(input);
        let expected: Vec<Vec<String>> = vec![
            vec!["root/a/2.txt".into(), "root/c/d/4.txt".into()],
            vec!["root/a/1.txt".into(), "root/c/3.txt".into()],
        ];

        assert_eq!(expected, output);
    }
}
