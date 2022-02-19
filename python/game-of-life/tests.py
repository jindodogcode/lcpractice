import unittest
from gameoflife import Solution


class TestGameOfLife(unittest.TestCase):
    def test_example_one(self):
        inp = [[0, 1, 0], [0, 0, 1], [1, 1, 1], [0, 0, 0]]
        Solution.gameOfLife(inp)
        expected = [[0, 0, 0], [1, 0, 1], [0, 1, 1], [0, 1, 0]]

        self.assertEqual(expected, inp)

    def test_example_two(self):
        inp = [[1, 1], [1, 0]]
        Solution.gameOfLife(inp)
        expected = [[1, 1], [1, 1]]

        self.assertEqual(expected, inp)


if __name__ == "__main__":
    unittest.main()
