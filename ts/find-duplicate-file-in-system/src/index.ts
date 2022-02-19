/**
 * # 609. Find Duplicate file in System
 *
 * Given a list `paths` of directory info, including the directory path, and all the files with 
 * contents in this dirctory, return _all the duplicate files in the file system in terms of their 
 * paths_. You may return the answer in __any order__.
 *
 * A group of duplicate files consists of at least two files that have the same content.
 *
 * A single directory info string in the input list has the following format:
 * - `"root/d1/d2/.../dm f1.txt(f1_content) f2.txt(f2_content) ... fn.txt(fn_content)"`
 *
 * It means there are `n` files `(f1.txt, f2.txt ... fn.txt)` with content `(f1_content, f2_content
 * ... fn_content)` respectively in the directory `"root/d1/d2/.../dm"`. Note that `n >= 1` and
 * `m >= 0`. If `m = 0`, it means the directory is just the root directory.
 *
 * The output is a list of groups of duplicate file paths. For each group, it contains all the file
 * paths of the files that have the same content. A file path is a string that has the following
 * format:
 * - `"directory_path/file_name.txt"`
 *
 * ## Constraints:
 * - `1 <= paths.length <= 2 * 10⁴`
 * - `1 <= paths[i].length <= 3000`
 * - `1 <= sum(paths[i].length) <= 5 * 10⁵`
 * - `paths[i]` consist of English letters, digits, `'/'`, `'.'`, `'('`, `')'`, and `' '`.
 * - You may assume no files or directories share the same name in the same directory.
 * - You may assume each given directory info represents a unique directory. A single blank space
 *   sperates the directory path and file info.
 *
 * ## Follow Up:
 * - Imagine you are given a real file system, how will you search files? DFS or BFS?
 * - If the file content is very large (GB levle), how will you modify your solution?
 * - If you can only read the file by 1kb each time, how will you modify your solution?
 * - What is the time complexity of your modified solution? What is the most time-consuming part
 *   and memory-consuming part of it? How to optimize?
 * - How to make sure the duplicated files you find are not false positive?
 */

/**
 * @param {string[]} paths
 * @return {string[][]}
 */
function findDuplicate(paths: string[]): string[][] {
  const contentPathMap: Map<string, string[]> = new Map();

  for (let path of paths) {
    let pathArr = path.split(" ");
    let dir = pathArr[0];

    for (let file of pathArr.slice(1)) {
      let fileArr = file.split("(");
      let filename = fileArr[0];
      let fileContent = fileArr[1].slice(0, fileArr[1].length-1);
      let contentEntry = contentPathMap.get(fileContent);


      if (contentEntry instanceof Array) {
        contentEntry.push(`${dir}/${filename}`);
      } else {
        contentPathMap.set(fileContent, new Array(`${dir}/${filename}`));
      }
    }
  }

  const result = [];

  for (let paths of contentPathMap.values()) {
    if (paths.length > 1) result.push(paths);
  }

  return result;
}

export { findDuplicate };
