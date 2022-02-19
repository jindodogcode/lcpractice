""" 609. Find Duplicate File in System
Given a list paths of directory info, including the directory path, and all
the files with contents in this directory, return all the duplicate files in
the file system in terms of their paths. You may return the answer in any
order.

A group of duplicate files consists of at least wo files that have the same
content.


A single directory info string in the input list hast the following format:
- "root/d1/d2/.../dm f1.txt(f1_content) f2.txt(f2_content) ...
  fn.txt(fn_content)"

It means there are n files (1.txt, f2.txt ... fn.txt) with content (f1_content,
f2_content ... fn_content) respectively in the directory "root/d1/d2/.../dm".
Note that n >= 1 and m >= 0. If m = 0, it means the directory is just the root
directory.

The output is a list of groups of duplicate file paths. For each group, it
contains all the file paths of the files that have the same content. A file
path is a string that has the following format:
- "directory_path/file_name.txt"
"""


class Solution:

    @staticmethod
    def findDuplicate(paths: "list[str]") -> "list[list[str]]":
        content_path_map: dict[str, list[str]] = dict()

        for path in paths:
            split = path.split(' ')
            dir_path = split[0]

            for file in split[1:]:
                parts = file.split('(')
                filename = parts[0]
                file_content = parts[1][0:-1]

                content_path_map.setdefault(file_content, list()).append(
                    "{}/{}".format(dir_path, filename))

        duplicates = [v for v in content_path_map.values() if len(v)
                      > 1]
        duplicates.reverse()
        return duplicates
