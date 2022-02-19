import unittest
from lib import Solution


class Duplicates(unittest.TestCase):
    def test_example_one(self):
        paths = [
            "root/a 1.txt(abcd) 2.txt(efgh)",
            "root/c 3.txt(abcd)",
            "root/c/d 4.txt(efgh)",
            "root 4.txt(efgh)",
        ]
        output = Solution.findDuplicate(paths)
        expected = [
            ["root/a/2.txt", "root/c/d/4.txt", "root/4.txt"],
            ["root/a/1.txt", "root/c/3.txt"],
        ]
        self.assertEqual(expected, output)

    def test_example_two(self):
        paths = [
            "root/a 1.txt(abcd) 2.txt(efgh)",
            "root/c 3.txt(abcd)",
            "root/c/d 4.txt(efgh)",
        ]
        output = Solution.findDuplicate(paths)
        expected = [
            ["root/a/2.txt", "root/c/d/4.txt"],
            ["root/a/1.txt", "root/c/3.txt"],
        ]

        self.assertEqual(expected, output)


if __name__ == "__main__":
    unittest.main()
