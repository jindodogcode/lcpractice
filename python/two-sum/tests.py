import unittest
from twosum import Solution


class TestTwoSum(unittest.TestCase):

    def test_example_one(self):
        nums = [2, 7, 11, 15]
        target = 9
        output = Solution.twoSum(nums, target)
        expected = [0, 1]

        self.assertEqual(expected, output)

    def test_example_two(self):
        nums = [3, 2, 4]
        target = 6
        output = Solution.twoSum(nums, target)
        expected = [1, 2]

        self.assertEqual(expected, output)

    def test_example_three(self):
        nums = [3, 3]
        target = 6
        output = Solution.twoSum(nums, target)
        expected = [0, 1]

        self.assertEqual(expected, output)


if __name__ == "__main__":
    unittest.main()
