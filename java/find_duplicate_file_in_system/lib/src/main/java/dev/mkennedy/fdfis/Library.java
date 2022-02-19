/**
 * <h1>609. Find Duplicate File in System</h1>
 *
 * Given a list <code>paths</code> of directory info, including the directory path, and all the 
 * files with contents in this directory, return <em>all the duplicate files in the file system in
 * terms of their paths</em>. You may return the answer in <strong>any order</strong>.
 * <p>
 * A group of duplicate files consists of at least two files that have the same content.
 * <p>
 * A single directory info string in the input list has the following format:
 * <ul>
 *  <li><code>"root/d1/d2/.../dm f1.txt(f1_content) f2.txt(f2_content) ... fn.txt(fn_content)"</code></li>
 * </ul>
 * <p>
 * It means there are <code>n</code> files <code>(f1.txt, f2.txt ... fn.txt)</code> with content
 * <code>(f1_content, f2_content ... fn_content)</code> respectively in the directory 
 * <code>"root/d1/d2/.../dm"</code>. Note that <code>n &gt;= 1</code> and <code>m &gt;= 0</code>.
 * If <code>m = 0</code>, it means the directory is just the root directory.
 * <p>
 * The output is a list of groups of duplicate file paths. For each group, it contains all the
 * file paths of the files that have the same content. A file path is a string that has the
 * following format:
 * <ul>
 *  <li><code>"directory_path/file_name.txt"</code></li>
 * </ul>
 *
 * <h2>Constraints:</h2>
 * <ul>
 *  <li><code>1 &lt;= paths.length &lt;= 2 * 10⁴</code></li>
 *  <li><code>1 &lt;= paths[i].length &lt;= 3000</code></li>
 *  <li><code>1 &lt;= sum(paths[i].length) &lt;= 5 * 10⁵</code></li>
 *  <li><code>paths[i]</code> consist of English letters, digits, <code>'/'</code>,
 *  <code>'.'</code>, <code>'('</code>, <code>')'</code>, and <code>' '</code>.</li>
 *  <li>You may assume no files or bdirectories share the same name in the same directory.</li>
 *  <li>You may assume each given directory info represents a unique directory. A single blank 
 *  space separates the directory path and file info.</li>
 * </ul>
 *
 * <h2>Follow up:</h2>
 * <ul>
 *  <li>Imagine you are given a real file system, how will you search files? DFS or BFS?</li>
 *  <li>If the file content is very large (GB level), how will you modify your solution?</li>
 *  <li>If you can only read the file by 1kb each time, how will you modify your solution?</li>
 *  <li>What is the time complexity of your modified solution? What is the most time-consuming part
 *  and memory-consuming part of it? How to optimize?</li>
 *  <li>How to make sure the duplicated files you find are not false positive?</li>
 * </ul>
 *
 */
package dev.mkennedy.fdfis;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.stream.Collectors;

public class Library {
    public boolean someLibraryMethod() {
        return true;
    }

    public static List<List<String>> findDuplicate(String[] paths) {
        HashMap<String, List<String>> contentPathMap = new HashMap<>();

        for (String path : paths) {
            String[] split = path.split(" ");
            if (split.length == 1) {
                continue;
            }
            String dir = split[0];

            for (int i = 1; i < split.length; i++) {
                String file = split[i];
                String[] parts = file.split("\\(");
                String filename = parts[0];
                String fileContent = parts[1].substring(0, parts[1].length() - 1);

                contentPathMap.computeIfAbsent(fileContent, v -> new ArrayList<>())
                        .add(String.format("%s/%s", dir, filename));
            }
        }

        return contentPathMap.values().stream().filter(v -> v.size() > 1).collect(Collectors.toList());
    }
}
